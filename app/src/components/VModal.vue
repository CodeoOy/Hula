<template>
	<div class="modal fade" ref="hulaModal" :id="'hulaModal' + modalID" :data-bs-backdrop="modalStatic ? 'static' : null">
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
export default {
	name: 'VModal',
	props: {	
		modalTitle: String,
		modalID: String,
		modalStatic: {
			type: Boolean,
			default: false
		}
	},
	updated() {
		this.$refs.hulaModal.addEventListener('hidden.bs.modal', () => {
			this.$emit('updatedModal')
		})
	}
};
</script>