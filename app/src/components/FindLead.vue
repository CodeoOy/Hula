<template>
	<div>
		<form action="#" v-on:submit.prevent="getProjects">
			<select class="mb-2 form-select" v-model="selected" aria-label="Choose pro">
				<option :value="{}" disabled>Choose the pro</option>
				<option v-for="user in users" :key="user.name" :value="{ user }">{{ user.firstname }} {{ user.lastname }}</option>
			</select>
			<button type="submit" class="btn btn-gradient mb-1">Search</button>
		</form>	
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
			getUserData(userid) { 
				fetch(`/api/users/${userid}`, {
					method: 'GET',
					headers: {"Content-Type": "application/json"}
				})
				.then((response) => response.json())
				.then(response => { 
					this.user = response;
				})    
			},
			getUsers() {
				fetch('api/users', {method: 'GET'})
				.then((response) => response.json())
				.then(response => { 
					this.users = response;
					this.$emit('datafetched', this.users)
				})    
				.catch((errors) => {    
					console.log(errors);
				})
			},
			getProjects() {
				fetch('api/projects', {method: 'GET'})
				.then((response) => response.json())
				.then(response => { 
					this.projects = response;
					this.$emit('leadsfetched', this.projects)
				})    
				.catch((errors) => {    
					console.log(errors);
				})
			}
		},
		mounted() {
			this.getUsers()
		}
	}
</script>