<template>
	<div class="p-3 rounded-2 content-box bg-dark text-light">
		<h2>Pro search results</h2>
		<transition name="fadeHeight">
			<table class="table table-dark table-striped text-light">
				<thead>
					<tr>
						<th scope="col">#</th>
						<th scope="col">First name</th>
						<th scope="col">Last name</th>
						<th scope="col">Available</th>
					</tr>
				</thead>
				<tbody>
					<tr v-for="(user, index) in users" :key="user.uid">
						<th scope="row">{{ index + 1 }}</th>
						<td>{{ user.firstname }}</td>
						<td>{{ user.lastname }}</td>
						<td>{{ user.available }}</td>
						<!--<a href="#" v-on:click="getUserData(user.uid)">{{user.firstname}} {{ user.lastname }}</a>-->
					</tr>
				</tbody>
			</table>
		</transition>
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