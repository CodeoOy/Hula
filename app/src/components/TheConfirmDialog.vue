<template>
	<div v-if='dialogs.length'>
		<VModal v-for="(dialog, index) in dialogs" :key='dialog.id' ref='modal' :showAtStart='true' :modalTitle="dialog.title" v-on:modal-hidden="removeDialog(index)">
			<button type="submit" class="btn btn-gradient mb-1" @click='confirm(dialog)'>Confirm</button>
		</VModal>
	</div>
</template>

<script>
	import VModal from './VModal.vue'
	
	export default {
		name: 'ConfirmDialog',

		components: {
			VModal,
		},

		methods: {
			removeDialog(index) {
				const dialog = this.dialogs.splice(index, 1)[0]
				dialog.resolve(false)
			},

			confirm(dialog) {
				dialog.resolve(true)
				this.$refs.modal.hide()
			},
		},
	}
</script>