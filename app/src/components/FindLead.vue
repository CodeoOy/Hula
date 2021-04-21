<template>
	<div>
		<a href="#" v-on:click="getUserData('d428a3a4-7813-4550-a3f3-36e363aab899')">Get a user</a>
		<p>{{ user.email }}</p>
		<button v-on:click="getProjects()" class="btn btn-primary text-white">Search for projects</button>
	</div>
</template>

<script>
	export default {
		name: 'FindLead',
		data() {
			return {
				user: {},
				project: {},
				projects: {},
			}
		},
		methods: {
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
			getProjectData: function(uid) { 
				fetch('api/user', {
					method: 'POST',
					headers: {"Content-Type": "application/json"},
					body: JSON.stringify({"uid": uid})
				})
				.then((response) => response.json())
				.then(response => { 
					console.log(response);
					this.project = response;
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
		}
	}
</script>