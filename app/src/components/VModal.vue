<template>
	<div class="modal fade" ref="modal" :data-bs-backdrop="modalBackdrop" :data-bs-keyboard='modalBackdrop != "static"'>
		<div class="modal-dialog modal-lg">
			<div class="modal-content shadow-lg" :class='$colorScheme.modal'>
				<div class="modal-header" v-if="modalTitle">
					<h2 class="modal-title">{{ modalTitle }}</h2>
					<button v-if='modalBackdrop != "static"' type="button" class="btn-close btn-close-white" data-bs-dismiss="modal" aria-label="Close"></button>
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
			modalBackdrop: {
				type: [Boolean, String],
				default: true,
				validator :value => typeof value == 'boolean' || value == 'static',
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
