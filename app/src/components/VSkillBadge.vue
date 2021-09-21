<template>
	<span
		class='badge badge-skill align-text-bottom'
		:class='classes'
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
			highlight: {
				type: Boolean,
				default: false,
			},
		},

		computed: {
			classes() {
				return {
					'badge-skill-mandatory': this.mandatory,
					'badge-skill-highlight': this.highlight,
				}
			},
		},
	}
</script>
