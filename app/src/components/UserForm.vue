<template>
	<div>
		<form action="#" @submit.prevent="onSubmit" v-if="'lastname' in user">
			<p v-if="errorsPresent" class="error">Please fill out all fields!</p>
			<div class="mb-2">
				<label class="form-label">First name</label>
				<input class="form-control" type="text" placeholder="Firstname" name="name" v-model="user.firstname" />
			</div>
			<div class="mb-2">
				<label class="form-label">Last name</label>
				<input class="form-control" type="text" placeholder="Lastname" name="name" v-model="user.lastname" />
			</div>
			<div class="mb-2 form-check">
				<label class="form-label">Available for work</label>
				<input type="checkbox" class="form-check-input" name="available" v-model="user.available" />
			</div>
			<button type="submit" class="btn btn-gradient">Save</button>
		</form>   
	</div>
</template>

<script>
export default {
	name: 'UserForm',
	props: {	
		user: {}
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