<template>
	<div v-if='modals.length'>
		<VModal
			:ref='id(modal)'
			v-for='modal in modals'
			:key='modal.id'
			:showAtStart='true'
			:title='modal.title'
			:backdrop='modal.backdrop'
			:size='modal.size'
			@modal-hidden='removeModal(modal)'>
			<component :is='modal.component' v-bind='modal.props' @success='onSuccess(modal, $event)' />
		</VModal>
	</div>
</template>

<script>
	import VModal from '@components/VModal.vue'
	
	export default {
		name: 'TheModal',

		components: {
			VModal,
		},

		methods: {
			id(modal) {
				return `modal-${modal.id}`
			},

			removeModal(modal) {
				this.modals.splice(this.modals.indexOf(modal), 1)
				modal.resolve(null)
			},

			onSuccess(modal, payload) {
				modal.resolve(payload)
				this.$refs[this.id(modal)].hide()
			},
		},
	}
</script>
