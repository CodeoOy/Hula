<template>
	<div>
		<form action="#" @submit.prevent="onSubmit" v-if="'lastname' in user">
			<p v-if="errorsPresent" class="error">Please fill out all fields!</p>
			<ul class="form-fields">
				<li class="form-field"><input type="text" placeholder="Your Name" name="name" v-model="user.firstname" /></li>
				<li class="form-field"><input type="text" placeholder="Your Name" name="name" v-model="user.lastname" /></li>
				<li class="form-field"><input type="checkbox" name="available" v-model="user.available" /></li>
				<li class="form-field"><input type="submit" class="button" value="Save" /></li>
			</ul>
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
		}
	}
};
</script>