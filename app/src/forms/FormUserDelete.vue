<template>
	<form v-on:submit="deleteUser">
		<p v-if="errorsPresent" class="error">Please fill out label!</p>
		<div class="mb-2">
			<label class="form-label">Pick user</label>
			<select class="form-select mb-2" id="User" aria-label="User" v-model="userToDelete">
				<option v-for="user in users" :key="user" :value="user.id">
					{{ user.firstname }} {{ user.lastname }}
				</option>
			</select>
		</div>
		<button type="submit" class="btn btn-gradient mb-1">Delete</button>
	</form> 
</template>

<script>
export default {
	name: 'UserDelete',
	data() {
		return {
			users: {},	
			userToDelete: ''	
		}
	},
	methods: {
		deleteUser() {
			fetch(`/api/users/${this.userToDelete}`, {
				method: 'DELETE', 
			})
			.catch((errors) => {
				this.$store.commit('errorHandling', errors)
			})
		},
		getAllUsers() {
			fetch('/api/users', {method: 'GET'})
			.then((response) => response.json())
			.then(response => { 
				this.users = response;
			})    
			.catch((errors) => {
				this.$store.commit('errorHandling', errors)
			})
		}
	},
	mounted() {
		this.getAllUsers()
	}
};
</script>