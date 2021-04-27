<template>
	<div>
		<form action=""></form>
		<select class="mb-2 form-select" aria-label="Choose project">
		<option selected>Choose the project</option>
			<option v-for="project in projects" :key="project.name" value="lol">{{ project.name }}</option>
		</select>
		<div class="mb-2 form-check">
			<label class="form-label">Availability</label>
			<input type="checkbox" class="form-check-input" name="available" />
		</div>
		<button v-on:click="getUsers('users')" class="btn btn-gradient">Search for pros</button>
	</div>
</template>

<script>
	export default {
		name: 'FindPro',
		data() {
			return {
				projects: {},
				user: {},
				users: {},
			}
		},
		methods: {
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
			},
			getProjectData: function(pid) { 
				fetch('api/project', {
					method: 'POST',
					headers: {"Content-Type": "application/json"},
					body: JSON.stringify({"pid": pid})
				})
				.then((response) => response.json())
				.then(response => { 
					console.log(response);
					this.project = response;
				})    
			},
			getUserData: function(uid) { 
				fetch('api/user', {
					method: 'POST',
					headers: {"Content-Type": "application/json"},
					body: JSON.stringify({"uid": uid})
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
			}
		},
		mounted() {
			this.getProjects()
		}
	}
</script>