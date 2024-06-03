use proc_macro::TokenStream;
use quote::quote;

// --------------------------------------------------------------------------------------------------------------------
// DATATYPE FUNCTIONALITY ---------------------------------------------------------------------------------------------
// --------------------------------------------------------------------------------------------------------------------

fn deserialize_datatype_enum(element_type: &syn::Ident, data: &syn::DataEnum) -> TokenStream {
  let element_type_string = element_type.to_string();
  let mut enum_arms: Vec<proc_macro2::TokenStream> = Vec::new();

  // Iterate through all possible enum variants
  for variant in &data.variants {
    // Perform any requested field renaming
    let variant_type = &variant.ident;
    let variant_type_strings = match variant.attrs.iter().find(|&attr| attr.path().is_ident("rename")) {
      Some(attr) => {
        if let Ok(list) = attr.meta.require_list() {
          list
            .parse_args_with(syn::punctuated::Punctuated::<syn::LitStr, syn::Token![,]>::parse_terminated)
            .unwrap()
            .iter()
            .map(|v| v.token().to_string().replace('"', ""))
            .collect()
        } else {
          attr
            .parse_args::<syn::LitStr>()
            .unwrap()
            .token()
            .to_string()
            .replace('"', "")
            .split(',')
            .map(String::from)
            .collect()
        }
      }
      None => vec![variant_type.to_string().to_lowercase()],
    };

    // Add a match arm for each type of variant
    for variant_type_str in &variant_type_strings {
      enum_arms.push(quote! { #variant_type_str => Ok(#element_type::#variant_type) });
    }
  }
  enum_arms.push(quote! { other => Err(format!("Unknown MusicXML variant for {}: {}", #element_type_string, other)) });

  // Generate the actual deserialization function
  TokenStream::from(quote! {
    impl DatatypeDeserializer for #element_type {
      fn deserialize(value: &str) -> Result<#element_type, String> {
        match value { #(#enum_arms),* }
      }
    }
  })
}

fn serialize_datatype_enum(element_type: &syn::Ident, data: &syn::DataEnum) -> TokenStream {
  let mut enum_arms: Vec<proc_macro2::TokenStream> = Vec::new();

  // Iterate through all possible enum variants
  for variant in &data.variants {
    // Perform any requested field renaming
    let variant_type = &variant.ident;
    let variant_type_string = match variant.attrs.iter().find(|&attr| attr.path().is_ident("rename")) {
      Some(attr) => {
        if let Ok(list) = attr.meta.require_list() {
          list
            .parse_args_with(syn::punctuated::Punctuated::<syn::LitStr, syn::Token![,]>::parse_terminated)
            .unwrap()
            .first()
            .unwrap()
            .token()
            .to_string()
            .replace('"', "")
        } else {
          attr
            .parse_args::<syn::LitStr>()
            .unwrap()
            .token()
            .to_string()
            .replace('"', "")
        }
      }
      None => variant_type.to_string().to_lowercase(),
    };

    // Add a match arm for this variant type
    enum_arms.push(quote! { #element_type::#variant_type => String::from(#variant_type_string) });
  }

  // Generate the actual serialization function
  TokenStream::from(quote! {
    impl DatatypeSerializer for #element_type {
      fn serialize(element: &#element_type) -> String {
        match element { #(#enum_arms),* }
      }
    }
  })
}

fn deserialize_datatype_tuple_struct(element_type: &syn::Ident, fields: &syn::FieldsUnnamed) -> TokenStream {
  // Deserialize first unnamed struct field
  let deserialized_field = match &fields.unnamed.first().unwrap().ty {
    syn::Type::Path(type_path) => quote! { value.parse::<#type_path>().map_err(|err| err.to_string())? },
    _ => panic!("Unknown MusicXML Element field type"),
  };

  // Generate the actual deserialization function
  TokenStream::from(quote! {
    impl DatatypeDeserializer for #element_type {
      fn deserialize(value: &str) -> Result<#element_type, String> {
        Ok(#element_type(#deserialized_field))
      }
    }
  })
}

fn serialize_datatype_tuple_struct(element_type: &syn::Ident, _fields: &syn::FieldsUnnamed) -> TokenStream {
  // Generate the actual serialization function
  TokenStream::from(quote! {
    impl DatatypeSerializer for #element_type {
      fn serialize(element: &#element_type) -> String {
        (*element).to_string()
      }
    }
  })
}

#[proc_macro_derive(DatatypeDeserialize, attributes(rename))]
pub fn deserialize_datatype(tokens: TokenStream) -> TokenStream {
  let ast: syn::DeriveInput = syn::parse(tokens).unwrap();
  match &ast.data {
    syn::Data::Struct(data) => match &data.fields {
      syn::Fields::Unnamed(unnamed_fields) => deserialize_datatype_tuple_struct(&ast.ident, unnamed_fields),
      _ => panic!("Unit structs are not supported in MusicXML"),
    },
    syn::Data::Enum(data) => deserialize_datatype_enum(&ast.ident, data),
    _ => panic!("Unions are unsupported in MusicXML datatypes"),
  }
}

#[proc_macro_derive(DatatypeSerialize, attributes(rename))]
pub fn serialize_datatype(tokens: TokenStream) -> TokenStream {
  let ast: syn::DeriveInput = syn::parse(tokens).unwrap();
  match &ast.data {
    syn::Data::Struct(data) => match &data.fields {
      syn::Fields::Unnamed(unnamed_fields) => serialize_datatype_tuple_struct(&ast.ident, unnamed_fields),
      _ => panic!("Unit structs are not supported in MusicXML"),
    },
    syn::Data::Enum(data) => serialize_datatype_enum(&ast.ident, data),
    _ => panic!("Unions are unsupported in MusicXML datatypes"),
  }
}


// --------------------------------------------------------------------------------------------------------------------
// ATTRIBUTE FUNCTIONALITY --------------------------------------------------------------------------------------------
// --------------------------------------------------------------------------------------------------------------------

fn deserialize_attribute_named_struct(element_type: &syn::Ident, fields: &syn::FieldsNamed) -> TokenStream {
  let mut deserialized_fields: Vec<proc_macro2::TokenStream> = Vec::new();
  let element_type_string = element_type.to_string();

  // Iterate through all named struct fields
  for field in &fields.named {
    // Perform any requested field renaming
    let field_name = field.ident.as_ref().unwrap();
    let field_name_string = field_name.to_string().replace("xml_", "xml:").replace("xlink_", "xlink:").replace("_", "-").replace("r#", "");

    // Deserialize field based on its type
    match &field.ty {
      syn::Type::Path(type_path) => {
        let field_details = type_path.path.segments.first().unwrap();
        match &field_details.ident {
          field_type if field_type == "Option" => {
            if let syn::PathArguments::AngleBracketed(details) = &field_details.arguments {
              if let syn::GenericArgument::Type(option_type) = details.args.first().unwrap() {
                if let syn::Type::Path(option_path) = option_type {
                  deserialized_fields.push(quote! {
                    #field_name: match attributes.iter().find(|&el| el.0 == #field_name_string) {
                      Some(attr) => Some(#option_path::deserialize(attr.1.as_str())?),
                      None => None,
                    }
                  });
                }
              }
            }
          },
          _ => {
            deserialized_fields.push(quote! {
              #field_name: match attributes.iter().find(|&el| el.0 == #field_name_string) {
                Some(attr) => #type_path::deserialize(attr.1.as_str())?,
                None => Err(format!("Missing required attribute for '{}': {}", #element_type_string, #field_name_string))?,
              }
            });
          },
        }
      },
      _ => panic!("Unknown MusicXML element attribute field type"),
    }
  }

  // Generate the actual deserialization function
  TokenStream::from(quote! {
    impl AttributeDeserializer for #element_type {
      fn deserialize(attributes: &Vec<(String, String)>) -> Result<#element_type, String> {
        Ok(#element_type { #(#deserialized_fields),* })
      }
    }
  })
}

fn serialize_attribute_named_struct(element_type: &syn::Ident, fields: &syn::FieldsNamed) -> TokenStream {
  let mut serialized_fields: Vec<proc_macro2::TokenStream> = Vec::new();

  // Iterate through all named struct fields
  for field in &fields.named {
    // Perform any requested field renaming
    let field_name = field.ident.as_ref().unwrap();
    let field_name_string = field_name.to_string().replace("xml_", "xml:").replace("xlink_", "xlink:").replace("r#", "").replace("_", "-");

    // Serialize field based on its type
    match &field.ty {
      syn::Type::Path(type_path) => {
        let field_details = type_path.path.segments.first().unwrap();
        match &field_details.ident {
          field_type if field_type == "Option" => {
            if let syn::PathArguments::AngleBracketed(details) = &field_details.arguments {
              if let syn::GenericArgument::Type(option_type) = details.args.first().unwrap() {
                if let syn::Type::Path(option_path) = option_type {
                  serialized_fields.push(quote! {
                    if let Some(data) = &element.#field_name {
                      attributes.push((String::from(#field_name_string), #option_path::serialize(data)));
                    }
                  });
                }
              }
            }
          },
          _ => {
            serialized_fields.push(quote! { attributes.push((String::from(#field_name_string), #type_path::serialize(&element.#field_name))); });
          },
        }
      }
      _ => panic!("Unknown MusicXML Element field type"),
    }
  }

  // Generate the actual serialization function
  TokenStream::from(quote! {
    impl AttributeSerializer for #element_type {
      fn serialize(element: &#element_type) -> Vec<(String, String)> {
        let mut attributes: Vec<(String, String)> = Vec::new();
        #(#serialized_fields)*;
        attributes
      }
    }
  })
}

#[proc_macro_derive(AttributeDeserialize, attributes(rename))]
pub fn deserialize_attribute(tokens: TokenStream) -> TokenStream {
  let ast: syn::DeriveInput = syn::parse(tokens).unwrap();
  match &ast.data {
    syn::Data::Struct(data) => match &data.fields {
      syn::Fields::Named(named_fields) => deserialize_attribute_named_struct(&ast.ident, named_fields),
      _ => panic!("Unit and tuple structs are not supported in MusicXML attributes"),
    },
    _ => panic!("Unions and enums are unsupported in MusicXML attributes"),
  }
}

#[proc_macro_derive(AttributeSerialize, attributes(rename))]
pub fn serialize_attribute(tokens: TokenStream) -> TokenStream {
  let ast: syn::DeriveInput = syn::parse(tokens).unwrap();
  match &ast.data {
    syn::Data::Struct(data) => match &data.fields {
      syn::Fields::Named(named_fields) => serialize_attribute_named_struct(&ast.ident, named_fields),
      _ => panic!("Unit and tuple structs are not supported in MusicXML attributes"),
    },
    _ => panic!("Unions and enums are unsupported in MusicXML attributes"),
  }
}


// --------------------------------------------------------------------------------------------------------------------
// CONTENT FUNCTIONALITY ----------------------------------------------------------------------------------------------
// --------------------------------------------------------------------------------------------------------------------

fn deserialize_content_named_struct(element_type: &syn::Ident, fields: &syn::FieldsNamed) -> TokenStream {
  let mut deserialized_fields: Vec<proc_macro2::TokenStream> = Vec::new();
  let element_type_string = element_type.to_string();

  // Iterate through all named struct fields
  for field in &fields.named {
    // Perform any requested field renaming
    let field_name = field.ident.as_ref().unwrap();
    let field_name_string = field_name.to_string().replace("_", "-");

    // Deserialize field based on its type
    match &field.ty {
      syn::Type::Path(type_path) => {
        let field_details = type_path.path.segments.first().unwrap();
        match &field_details.ident {
          field_type if field_type == "Option" => {
            if let syn::PathArguments::AngleBracketed(details) = &field_details.arguments {
              if let syn::GenericArgument::Type(option_type) = details.args.first().unwrap() {
                if let syn::Type::Path(option_path) = option_type {
                  deserialized_fields.push(quote! {
                    #field_name: match elements.iter().find(|&el| el.name == #field_name_string) {
                        Some(element) => Some(#option_path::deserialize(element)?),
                        None => None,
                      }
                  });
                }
              }
            }
          },
          field_type if field_type == "Vec" => {
            if let syn::PathArguments::AngleBracketed(details) = &field_details.arguments {
              if let syn::GenericArgument::Type(vec_type) = details.args.first().unwrap() {
                if let syn::Type::Path(vec_path) = vec_type {
                  deserialized_fields.push(quote! { #field_name: elements.iter().filter_map(|el| if el.name == #field_name_string { #vec_path::deserialize(el).ok() } else { None }).collect::<Vec<_>>() });
                }
              }
            }
          },
          _ => {
            deserialized_fields.push(quote! { #field_name: #type_path::deserialize(match elements.iter().find(|&el| el.name == #field_name_string) { Some(val) => val, None => Err(format!("Missing required element <{}> in <{}>", #field_name_string, #element_type_string))? })? });
          }
        }
      }
      _ => panic!("Unknown MusicXML content field type"),
    }
  }

  // Generate the actual deserialization function
  TokenStream::from(quote! {
    impl ContentDeserializer for #element_type {
      fn deserialize(elements: &Vec<XmlElement>) -> Result<#element_type, String> {
        Ok(#element_type { #(#deserialized_fields),* })
      }
    }
  })
}

fn serialize_content_named_struct(element_type: &syn::Ident, fields: &syn::FieldsNamed) -> TokenStream {
  let mut serialized_fields: Vec<proc_macro2::TokenStream> = Vec::new();

  // Iterate through all named struct fields
  for field in &fields.named {
    // Perform any requested field renaming
    let field_name = field.ident.as_ref().unwrap();

    // Serialize field based on its type
    match &field.ty {
      syn::Type::Tuple(_type_tuple) => {},
      syn::Type::Path(type_path) => {
        let field_details = type_path.path.segments.first().unwrap();
        match &field_details.ident {
          field_type if field_type == "Option" => {
            if let syn::PathArguments::AngleBracketed(details) = &field_details.arguments {
              if let syn::GenericArgument::Type(option_type) = details.args.first().unwrap() {
                if let syn::Type::Path(option_path) = option_type {
                  serialized_fields.push(quote! {
                    if let Some(data) = &element.#field_name { elements.push(#option_path::serialize(data)); }
                  });
                }
              }
            }
          },
          field_type if field_type == "Vec" => {
            if let syn::PathArguments::AngleBracketed(details) = &field_details.arguments {
              if let syn::GenericArgument::Type(vec_type) = details.args.first().unwrap() {
                if let syn::Type::Path(vec_path) = vec_type {
                  serialized_fields.push(quote! {
                    for child in &element.#field_name { elements.push(#vec_path::serialize(child)); }
                  });
                }
              }
            }
          },
          _ => {
            serialized_fields.push(quote! { elements.push(#type_path::serialize(&element.#field_name)); });
          }
        }
      }
      _ => panic!("Unknown MusicXML Element field type"),
    }
  }

  // Generate the actual serialization function
  TokenStream::from(quote! {
    impl ContentSerializer for #element_type {
      fn serialize(element: &Self) -> Vec<XmlElement> {
        let mut elements: Vec<XmlElement> = Vec::new();
        #(#serialized_fields)*;
        elements
      }
    }
  })
}

#[proc_macro_derive(ContentDeserialize, attributes(rename))]
pub fn deserialize_content(tokens: TokenStream) -> TokenStream {
  let ast: syn::DeriveInput = syn::parse(tokens).unwrap();
  match &ast.data {
    syn::Data::Struct(data) => match &data.fields {
      syn::Fields::Named(named_fields) => deserialize_content_named_struct(&ast.ident, named_fields),
      _ => panic!("Unit and tuple structs are not supported in MusicXML content"),
    },
    _ => panic!("Unions and enums are unsupported in MusicXML content"),
  }
}

#[proc_macro_derive(ContentSerialize, attributes(rename))]
pub fn serialize_content(tokens: TokenStream) -> TokenStream {
  let ast: syn::DeriveInput = syn::parse(tokens).unwrap();
  match &ast.data {
    syn::Data::Struct(data) => match &data.fields {
      syn::Fields::Named(named_fields) => serialize_content_named_struct(&ast.ident, named_fields),
      _ => panic!("Unit and tuple structs are not supported in MusicXML content"),
    },
    _ => panic!("Unions and enums are unsupported in MusicXML content"),
  }
}


// --------------------------------------------------------------------------------------------------------------------
// ELEMENT FUNCTIONALITY ----------------------------------------------------------------------------------------------
// --------------------------------------------------------------------------------------------------------------------

fn deserialize_element_enum(element_type: &syn::Ident, data: &syn::DataEnum) -> TokenStream {
  let element_type_string = element_type.to_string();
  let mut enum_arms: Vec<proc_macro2::TokenStream> = Vec::new();

  // Iterate through all possible enum variants
  for variant in &data.variants {
    // Perform any requested field renaming
    let variant_type = &variant.ident;
    let variant_type_strings = match variant.attrs.iter().find(|&attr| attr.path().is_ident("rename")) {
      Some(attr) => {
        if let Ok(list) = attr.meta.require_list() {
          list
            .parse_args_with(syn::punctuated::Punctuated::<syn::LitStr, syn::Token![,]>::parse_terminated)
            .unwrap()
            .iter()
            .map(|v| v.token().to_string().replace('"', ""))
            .collect()
        } else {
          attr
            .parse_args::<syn::LitStr>()
            .unwrap()
            .token()
            .to_string()
            .replace('"', "")
            .split(',')
            .map(String::from)
            .collect()
        }
      }
      None => vec![variant_type.to_string().to_lowercase()],
    };

    // Add a match arm for each type of variant
    for variant_type_str in &variant_type_strings {
      enum_arms.push(
        quote! { #variant_type_str => Ok(#element_type::#variant_type(#variant_type::deserialize(element)?)) },
      );
    }
  }
  enum_arms.push(quote! { other => Err(format!("Unknown MusicXML variant for {}: {}", #element_type_string, other)) });

  // Generate the actual deserialization function
  TokenStream::from(quote! {
    impl ElementDeserializer for #element_type {
      fn deserialize(element: &XmlElement) -> Result<#element_type, String> {
        match element.name.as_ref() { #(#enum_arms),* }
      }
    }
  })
}

fn serialize_element_enum(element_type: &syn::Ident, data: &syn::DataEnum) -> TokenStream {
  let mut enum_arms: Vec<proc_macro2::TokenStream> = Vec::new();

  // Iterate through all possible enum variants
  for variant in &data.variants {
    // Perform any requested field renaming
    let variant_type = &variant.ident;
    let variant_type_string = match variant.attrs.iter().find(|&attr| attr.path().is_ident("rename")) {
      Some(attr) => {
        if let Ok(list) = attr.meta.require_list() {
          list
            .parse_args_with(syn::punctuated::Punctuated::<syn::LitStr, syn::Token![,]>::parse_terminated)
            .unwrap()
            .first()
            .unwrap()
            .token()
            .to_string()
            .replace('"', "")
        } else {
          attr
            .parse_args::<syn::LitStr>()
            .unwrap()
            .token()
            .to_string()
            .replace('"', "")
        }
      }
      None => variant_type.to_string().to_lowercase(),
    };

    // Add a match arm for this variant type
    enum_arms.push(quote! {
      #element_type::#variant_type(var) => {
        let mut result = #variant_type::serialize(var);
        result.name = String::from(#variant_type_string);
        result
      }
    });
  }

  // Generate the actual serialization function
  TokenStream::from(quote! {
    impl ElementSerializer for #element_type {
      fn serialize(element: &Self) -> XmlElement {
        match element { #(#enum_arms),* }
      }
    }
  })
}

fn deserialize_element_named_struct(element_type: &syn::Ident, fields: &syn::FieldsNamed) -> TokenStream {
  let mut deserialized_fields: Vec<proc_macro2::TokenStream> = Vec::new();

  // Iterate through all named struct fields
  for field in &fields.named {
    let field_name = field.ident.as_ref().unwrap();

    // Deserialize field based on its type
    match &field.ty {
      syn::Type::Tuple(_type_tuple) => deserialized_fields.push(quote! { #field_name: () }),
      syn::Type::Path(type_path) => {
        let field_details = type_path.path.segments.first().unwrap();
        match &field_details.ident {
          field_type if field_type == "Vec" => {
            if let syn::PathArguments::AngleBracketed(details) = &field_details.arguments {
              if let syn::GenericArgument::Type(vec_type) = details.args.first().unwrap() {
                if let syn::Type::Path(vec_path) = vec_type {
                  deserialized_fields.push(quote! { #field_name: element.elements.iter().filter_map(|el| #vec_path::deserialize(el).ok()).collect::<Vec<_>>() });
                }
              }
            }
          }
          _ => {
            if field_name == "attributes" {
              deserialized_fields.push(quote! { #field_name: #type_path::deserialize(&element.attributes)? });
            } else if field.attrs.iter().find(|&attr| attr.path().is_ident("flatten")).is_some() {
              deserialized_fields.push(quote! { #field_name: #type_path::deserialize(&element.elements)? });
            } else {
              deserialized_fields.push(quote! { #field_name: #type_path::deserialize(element.text.as_str())? });
            }
          }
        }
      }
      _ => panic!("Unknown MusicXML Element field type"),
    }
  }

  // Generate the actual deserialization function
  TokenStream::from(quote! {
    impl ElementDeserializer for #element_type {
      fn deserialize(element: &XmlElement) -> Result<#element_type, String> {
        Ok(#element_type { #(#deserialized_fields),* })
      }
    }
  })
}

fn serialize_element_named_struct(element_type: &syn::Ident, element_type_name: &String, fields: &syn::FieldsNamed) -> TokenStream {
  let mut serialized_fields: Vec<proc_macro2::TokenStream> = Vec::new();

  // Iterate through all named struct fields
  for field in &fields.named {
    let field_name = field.ident.as_ref().unwrap();

    // Serialize field based on its type
    match &field.ty {
      syn::Type::Tuple(_type_tuple) => {},
      syn::Type::Path(type_path) => {
        let field_details = type_path.path.segments.first().unwrap();
        match &field_details.ident {
          field_type if field_type == "Vec" => {
            if let syn::PathArguments::AngleBracketed(details) = &field_details.arguments {
              if let syn::GenericArgument::Type(vec_type) = details.args.first().unwrap() {
                if let syn::Type::Path(vec_path) = vec_type {
                  serialized_fields.push(quote! { elements: element.content.iter().map(|el| #vec_path::serialize(el)).collect::<Vec<_>>() });
                }
              }
            }
          },
          _ => {
            if field_name == "attributes" {
              serialized_fields.push(quote! { attributes: #type_path::serialize(&element.attributes) });
            } else if field.attrs.iter().find(|&attr| attr.path().is_ident("flatten")).is_some() {
              serialized_fields.push(quote! { elements: #type_path::serialize(&element.content) });
            } else {
              serialized_fields.push(quote! { text: #type_path::serialize(&element.content) });
            }
          },
        }
      },
      _ => panic!("Unknown MusicXML Element field type"),
    }
  }
  serialized_fields.push(quote! { ..Default::default()});

  // Generate the actual serialization function
  TokenStream::from(quote! {
    impl ElementSerializer for #element_type {
      fn serialize(element: &#element_type) -> XmlElement {
        XmlElement {
          name: String::from(#element_type_name),
          #(#serialized_fields),*
        }
      }
    }
  })
}

#[proc_macro_derive(ElementDeserialize, attributes(rename, flatten))]
pub fn deserialize_element(tokens: TokenStream) -> TokenStream {
  let ast: syn::DeriveInput = syn::parse(tokens).unwrap();
  match &ast.data {
    syn::Data::Struct(data) => match &data.fields {
      syn::Fields::Named(named_fields) => deserialize_element_named_struct(&ast.ident, named_fields),
      _ => panic!("Unit and tuple structs are not supported in MusicXML elements"),
    },
    syn::Data::Enum(data) => deserialize_element_enum(&ast.ident, data),
    _ => panic!("Unions are unsupported in MusicXML elements"),
  }
}

#[proc_macro_derive(ElementSerialize, attributes(rename))]
pub fn serialize_element(tokens: TokenStream) -> TokenStream {
  // Fetch the actual or renamed name of the element to serialize
  let ast: syn::DeriveInput = syn::parse(tokens).unwrap();
  let element_string = match ast.attrs.iter().find(|&attr| attr.path().is_ident("rename")) {
    Some(attr) => attr
      .parse_args::<syn::LitStr>()
      .unwrap()
      .token()
      .to_string()
      .replace('"', ""),
    None => ast.ident.to_string().to_lowercase(),
  };

  // Perform the actual data serialization
  match &ast.data {
    syn::Data::Struct(data) => match &data.fields {
      syn::Fields::Named(named_fields) => serialize_element_named_struct(&ast.ident, &element_string, named_fields),
      _ => panic!("Unit and tuple structs are not supported in MusicXML elements"),
    },
    syn::Data::Enum(data) => serialize_element_enum(&ast.ident, data),
    _ => panic!("Unions are unsupported in MusicXML elements"),
  }
}
