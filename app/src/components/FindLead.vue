<template>
	<div>
		<select class="mb-2 form-select" v-model="selected" aria-label="Choose project">
		<option selected>Choose the pro</option>
			<option v-for="user in users" :key="user.name" v-bind:value="{ user }">{{ user.firstname }} {{ user.lastname }}</option>
		</select>
		<button v-on:click="getProjects()" class="btn btn-gradient ">Search for projects</button>
	</div>
</template>

<script>
	export default {
		name: 'FindLead',
		data() {
			return {
				users: {},
				project: {},
				projects: {},
				selected: {},
			}
		},
		methods: {
			getUserData: function(userid) { 
				fetch(`http://localhost:8086/api/user/${userid}`, {
					method: 'GET',
					headers: {"Content-Type": "application/json"}
				})
				.then((response) => response.json())
				.then(response => { 
					console.log(response);
					this.user = response;
				})    
			},
			getUsers: function() {
				fetch('api/users', {method: 'GET'})
				.then((response) => response.json())
				.then(response => { 
					console.log(response);
					this.users = response;
					this.$emit('datafetched', this.users)
				})    
				.catch((errors) => {    
					console.log("Could not get data");
					console.log(errors);
				})
			},
			getProjects: function() {
				fetch('api/projects', {method: 'GET'})
				.then((response) => response.json())
				.then(response => { 
					console.log(response);
					this.projects = response;
					this.$emit('datafetched', this.projects)
				})    
				.catch((errors) => {    
					console.log("Could not get data");
					console.log(errors);
				})
			}
		},
		mounted() {
			this.getUsers()
		}
	}
</script>