<template>
	<span v-html='highlight' />
</template>

<script>
	export default {
		name: 'VHighlight',

		props: {
			text: {
				type: String,
				required: true,
			},

			keywords: {
				type: [String, Array],
				required: true,
			}
		},

		computed: {
			highlightPattern() {
				return new RegExp(this.keywords.join('|'), 'gi')
			},

			highlight() {
				return this.keywords.length
					? this.text.replace(this.highlightPattern, '<mark>$&</mark>')
					: this.text
			},
		},

	}
</script>
