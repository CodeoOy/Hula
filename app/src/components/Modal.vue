<template>
	<div class="modal fade" id="hulaModal" v-bind:class="{ 'show db': show_modal, '': !show_modal }">
		<div class="modal-dialog">
			<div class="modal-content p-3 rounded-2 content-box bg-dark text-light">
				<div>
					<h2>{{ modal_title }}</h2>
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
		show_modal: false,
		modal_title: '',
		modal_content: '',
	},
	data() {
		return {
			errorsPresent: false
		};
	}
};
</script>