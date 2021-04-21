<template>
	<div>
		<a href="#" v-on:click="getProjectData('668ef6f0-fec1-4b0a-8fe2-cdbcd11ccf0b')">Get a project</a>
		<p>{{ project.name }}</p>
		<button v-on:click="getUsers('users')" class="btn btn-primary text-white">Search for pros</button>
	</div>
</template>

<script>
	export default {
		name: 'FindPro',
		data() {
			return {
				project: {},
				user: {},
				users: {},
			}
		},
		methods: {
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
			getUsers: function(table) {
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
		}
	}
</script>