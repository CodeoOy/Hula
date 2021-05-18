<template>
	<div class="modal fade" v-bind:class="{ 'show db': showModal, '': !showModal }">
		<div class="modal-dialog">
			<div class="modal-content p-3 rounded-2 content-box bg-dark text-light">
				<div>
					<h2>Modal title</h2>
					<slot></slot>
				</div>
			</div>
		</div>
	</div>
</template>

<script>
export default {
	name: 'SkillScope',
	props: {	
		user: {},
		showModal: false,
	},
	data() {
		return {
			errorsPresent: false
		};
	},
	methods: {
		onSubmit: function() {
			if (this.user.lastname === '') {
				this.errorsPresent = true;
				this.$flashMessage.show({
					type: 'error',
					title: 'Please fill out name.',
					time: 1000
				});
			} else {
				this.$emit('formsent', this.user);
			}
		},
	}
};
</script>