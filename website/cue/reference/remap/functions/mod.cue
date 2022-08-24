package metadata

remap: functions: mod: {
	category: "Number"
	description: """
		Calculates the remainder of `value` divided by `modulus`.
		"""

	arguments: [
		{
			name:        "value"
			description: "The value the `modulus` is applied to."
			required:    true
			type: ["number"]
		},
		{
			name:        "modulus"
			description: "The `modulus` value."
			required:    true
			type: ["number"]
		}
	]
	internal_failure_reasons: [
		"`value` isn't an integer or float",
		"`modulus` isn't an integer or float",
		"`modulus` is equal to 0"
	]
	return: types: ["string"]

	examples: [
		{
			title: "Calculate the remainder of two integers"
			source: #"""
				remainder = mod(5, 2)
				"""#
			return: 1
		},
	]
}
