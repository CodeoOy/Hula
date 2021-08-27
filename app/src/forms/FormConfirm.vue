<template>
	<button type="submit" class="btn btn-gradient mb-1" @click='confirm'>{{ button }}</button>
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