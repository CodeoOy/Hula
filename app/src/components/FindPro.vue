<template>
	<div>
		<p>First pick a project</p>
		<button v-on:click="getList('users')" class="btn btn-primary text-white">Search for pros</button>
	</div>
</template>

<script>
	export default {
		name: 'FindPro',
		data() {
			return {
				user: {},
				users: {},
			}
		},
		methods: {
			getUserData: function(uid) { 
				fetch('api/query', {
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
			getList: function(table) {
				fetch(`api/${table}`, {method: 'GET'})
				.then((response) => response.json())
				.then(response => { 
					console.log(response);
					if (table == 'users') {
						this.users = response;
					} else {
						this.items = response;
					}
				})    
				.catch((errors) => {    
					console.log("Could not get data");
					console.log(errors);
				})
			}
		}
	}
</script>