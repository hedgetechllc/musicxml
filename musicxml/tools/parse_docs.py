import json

def add_attributes(output, attributes, attribute_groups, attribute_group):
	output.extend([attribute for attribute in attributes if attribute['fields']['attribute_group'] == attribute_group])
	for child in [group['fields']['child_groups'] for group in attribute_groups if group['pk'] == attribute_group][0]:
		add_attributes(output, attributes, attribute_groups, child)


with open('musicxmldoc.json') as file:
	data = json.load(file)

	datatypes = [element for element in data if element['model'] == 'spec.datatype']
	datatype_options = [element for element in data if element['model'] == 'spec.datatypeoption']
	elements = [element for element in data if element['model'] == 'spec.xmlelement' and not element['fields']['is_abstract_element']]
	attributes = [element for element in data if element['model'] == 'spec.xmlattribute']
	attribute_groups = [element for element in data if element['model'] == 'spec.xmlattributegroup']

	for element in elements:
		attrs = []
		content = []
		name = element['fields']['name']
		description = element['fields']['description']

		while element['fields']['base_element'] is not None:
			attrs.extend([attribute for attribute in attributes if attribute['fields']['element'] == element['pk']])
			for group in element['fields']['attribute_groups']:
				add_attributes(attrs, attributes, attribute_groups, group)
			element = [el for el in data if el['model'] == 'spec.xmlelement' and el['pk'] == element['fields']['base_element']][0]
			if not description:
				description = element['fields']['description']
		attrs.extend([attribute for attribute in attributes if attribute['fields']['element'] == element['pk']])
		for group in element['fields']['attribute_groups']:
			add_attributes(attrs, attributes, attribute_groups, group)
		attrs.sort(key=lambda x: x['fields']['name'])

		if element['fields']['content_data_type'] is not None:
			''.join([word.capitalize() for word in [dt['fields']['name'] for dt in datatypes if dt['pk'] == element['fields']['content_data_type']][0].replace('-', ' ').replace(':', ' ').split(' ')])

		docs = '\n\n\n' + name + '\n/// ' + '\n/// \n/// '.join('\n'.join(description.split('\r\n')).split('\n\n')) + '\n/// \n'
		docs += '/// ## Content\n' if content else '/// ## Content\n/// Always empty\n'
		docs += '/// \n/// ## Attributes\n/// | Name | Type | Required? | Description |\n' if attrs else '/// ## Attributes\n/// None'

		for attribute in attrs:
			attr_name = attribute['fields']['name']
			attr_description = '\\n'.join('\n'.join(attribute['fields']['description'].split('\r\n')).split('\n\n'))
			attr_required = 'Yes' if attribute['fields']['is_required'] else 'No'
			attr_datatype = ''.join([word.capitalize() for word in [dt['fields']['name'] for dt in datatypes if dt['pk'] == attribute['fields']['data_type']][0].replace('-', ' ').replace(':', ' ').split(' ')])
			docs += '/// | ' + attr_name + ' | [' + attr_datatype + '][crate::datatypes::' + attr_datatype + '] | ' + attr_required + ' | ' + attr_description + ' |\n'

		docs = docs.replace('&lt;', '`<').replace('&gt;', '>`')

		print(docs)
