<template>
	<div class='d-flex justify-content-between'>
		<button type='submit' class='btn btn-secondary' @click='confirm(false)'>{{ cancel }}</button>
		<button type='submit' class='btn btn-primary gradient' @click='confirm(true)'>{{ ok }}</button>
	</div>
</template>

<script>
	export default {
		name: 'FormConfirm',

		props: {
			ok: {
				type: String,
				default: 'OK',
			},
			cancel: {
				type: String,
				default: 'Cancel',
			},
		},

		methods: {
			confirm(choice) {
				this.$emit('success', choice)
			},
		},
	}
</script>
