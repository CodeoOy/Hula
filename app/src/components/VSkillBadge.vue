<template>
	<span
		class='badge badge-skill align-text-bottom'
		:class='{ "badge-skill-mandatory": this.mandatory }'
		:style='{ "--progress": `${percentage}%` || 0 }'
	>
		{{ label }}
	</span>
</template>

<script>
	export default {
		name: 'VSkillBadge',

		props: {
			label: {
				type: String,
				required: true
			},
			mandatory: {
				type: Boolean,
				default: false,
			},
			percentage: {
				type: [Number, null],
				default: 0,
			},
		},
	}
</script>