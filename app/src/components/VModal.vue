<template>
	<div class="modal fade" ref="modal" :data-bs-backdrop="backdrop" :data-bs-keyboard='backdrop != "static"'>
		<div class="modal-dialog modal-lg">
			<div class="modal-content shadow-lg" :class='$colorScheme.modal'>
				<div class="modal-header" v-if="title">
					<div class="modal-title h2">{{ title }}</div>
					<button v-if='backdrop != "static"' type="button" class="btn-close" :class='$colorScheme.modalClose' data-bs-dismiss="modal" aria-label="Close"></button>
				</div>
				<div ref='body' class="modal-body">
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
			title: String,
			backdrop: {
				type: [Boolean, String],
				default: true,
				validator :value => typeof value == 'boolean' || value == 'static',
			},
			showAtStart: false,
		},

		mounted() {
			this.modal = Modal.getOrCreateInstance(this.$refs.modal)

			this.$refs.modal.addEventListener('hide.bs.modal', () => { this.$emit('modal-hiding') })
			this.$refs.modal.addEventListener('hidden.bs.modal', () => { this.$emit('modal-hidden') })
			this.$refs.modal.addEventListener('show.bs.modal', () => { this.$emit('modal-showing') })
			this.$refs.modal.addEventListener('shown.bs.modal', () => {
				this.$emit('modal-shown')
				const input = this.$refs.body.querySelector('input, select, button, a')
				if (input) input.focus()
			})

			if (this.showAtStart) this.modal.show()
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
