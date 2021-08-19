<template>
	<div class="modal fade" ref="modal" :id="'hulaModal' + modalID" :data-bs-backdrop="modalStatic ? 'static' : null">
		<div class="modal-dialog modal-lg">
			<div class="modal-content rounded-2 content-box bg-dark text-light">
				<div class="modal-header" v-if="modalStatic == false">
					<h2 class="modal-title">{{ modalTitle }}</h2>
					<button type="button" class="btn-close btn-close-white" data-bs-dismiss="modal" aria-label="Close"></button>
				</div>
				<div class="modal-body">
					<slot></slot>
				</div>
			</div>
		</div>
	</div>
</template>

<script>
	import { Modal } from 'bootstrap'

	export default {
		name: 'VModal',
		
		props: {
			modalTitle: String,
			modalID: String,
			modalStatic: {
				type: Boolean,
				default: false
			},
			showAtStart: false,
		},

		mounted() {
			this.modal = Modal.getOrCreateInstance(this.$refs.modal)
			if (this.showAtStart) this.modal.show()

			this.$refs.modal.addEventListener('hide.bs.modal', () => { this.$emit('modal-hiding') })
			this.$refs.modal.addEventListener('hidden.bs.modal', () => { this.$emit('modal-hidden') })
			this.$refs.modal.addEventListener('show.bs.modal', () => { this.$emit('modal-showing') })
			this.$refs.modal.addEventListener('shown.bs.modal', () => { this.$emit('modal-shown') })
		},

		methods: {
			show() {
				this.modal.show()
			},

			hide() {
				this.modal.hide()
			}
		}
	}
</script>
