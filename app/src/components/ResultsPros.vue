<template>
	<div class="p-3 rounded-2 content-box bg-dark text-light">
		<h2>Pro search results</h2>
		<ul>
			<li v-for="user in users" :key="user.uid">
				<a href="#" v-on:click="getUserData(user.uid)">{{user.email}}</a>
			</li>
		</ul>
	</div>
</template>

<script>
	export default {
		name: 'ResultsPros',
		data() {
			return {
				message: "moro",
			}
		},
		props: {
			users: {}
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
			}
		}
	}
</script>