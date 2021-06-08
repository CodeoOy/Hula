<template>
	<div class="modal fade" id="hulaModal">
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
	name: 'VModal',
	props: {	
		modal_title: String,
	}
};
</script>