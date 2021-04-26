<template>
	<div>
		<a href="#" v-on:click="getUserData('996df290-ad97-4a93-b6d0-10e3c3f0aaea')">Get a user</a>
		<p>{{ user.email }}</p>
		<button v-on:click="getProjects()" class="btn btn-gradient ">Search for projects</button>
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