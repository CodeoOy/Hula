<template>
	<div>
		<p><a href="#" v-on:click="getList('users')">Get all users</a></p>
		<p><a href="#" v-on:click="getList('projects')">Get all projects</a></p>
		<p>{{ item[0] }}</p>
		<ul>
			<li v-for="item in items" :key="item.pid">
				{{item.name}}
			</li>
		</ul>
		<ul>
			<li v-for="user in users" :key="user.email">
				<a href="#" v-on:click="getUserData(user.uid)">{{user.email}}</a>
			</li>
		</ul>
	</div>
</template>

<script>
	export default {
		name: 'List',
		data() {
			return {
				user: {},
				users: {},
				item: {},
				items: {}
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
			/*
			getItemData: function(uid) { 
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
			*/
			getList: function(table) {
				fetch(`api/${table}`, {method: 'GET'})
				.then((response) => response.json())
				.then(response => { 
					console.log(response);
					this.users = response;
				})    
				.catch((errors) => {    
					console.log("Could not get data");
					console.log(errors);
				})
			}
		}
	}
</script>