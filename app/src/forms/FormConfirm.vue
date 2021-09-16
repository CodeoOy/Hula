<template>
	<button type='submit' class='btn btn-primary gradient' @click='confirm'>{{ button }}</button>
</template>

<script>
	export default {
		name: 'FormConfirm',

		props: {
			button: {
				type: String,
				default: 'Confirm',
			}
		},

		methods: {
			async confirm() {
				this.$emit('success', true)
			},
		},
	}
</script>