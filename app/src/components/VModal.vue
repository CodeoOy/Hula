<template>
	<div class="modal fade" id="hulaModal">
		<div class="modal-dialog">
			<div class="modal-content rounded-2 content-box bg-dark text-light">
				<div class="modal-header">
					<h2 class="modal-title">{{ modal_title }}</h2>
					<button type="button" class="close" data-bs-toggle="modal" data-bs-target="#hulaModal" aria-label="Close">
						<span aria-hidden="true">&times;</span>
					</button>
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
		modal_title: String,
	}
};
</script>