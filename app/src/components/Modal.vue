<template>
	<div class="modal fade" v-bind:class="{ 'show db': showModal, '': !showModal }">
		<div class="modal-dialog">
			<div class="modal-content p-3 rounded-2 content-box bg-dark text-light">
				<div>
					<h2>{{ modalTitle }}</h2>
					<slot></slot>
				</div>
			</div>
		</div>
	</div>
</template>

<script>
export default {
	name: 'Modal',
	props: {	
		showModal: false,
		modalTitle: '',
	},
	data() {
		return {
			errorsPresent: false
		};
	}
};
</script>