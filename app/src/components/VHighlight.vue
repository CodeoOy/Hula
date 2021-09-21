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

			pattern: RegExp,
		},

		computed: {
			highlight() {
				return this.pattern
					? this.text.replace(this.pattern, '<mark>$&</mark>')
					: this.text
			},
		},

	}
</script>
