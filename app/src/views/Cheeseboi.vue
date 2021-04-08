<template>
	<div class="container">
		<div class="row mt-4">
			<div class="col">
				<div class="tropical-box rounded-2 p-3">
					<a href="#" v-on:click="getRustData">Test link. May produce a treefloof.</a>
					<p v-if="message.length">{{ message }}</p>
					<p v-if="registration_data.length">{{ registration_data }}</p>
				</div>
			</div>
		</div>
	</div>
</template>

<script>
	import { useRoute } from 'vue-router'
	export default {
		name: 'Cheeseboi',
		data() {
			return {
				message: JSON,
				registration_data: String,
			}
		},
		methods: {
			getRustData: function() {
				console.log("getRustData fired.");  
				fetch('/omnom', {method: 'GET'})
				.then((response) => response.json())
				.then(response => { 
					console.log(response);
					this.message = response;
					this.$flashMessage.show({
						type: 'success',
						title: 'Successfully fetched data',
						time: 1000
					});
				})
				.catch((errors) => {    
					console.log("Could not get data");
					console.log(errors);
					//this.$router.push('/create'); 
				})
			},
			confirm_registration: function() {  
				fetch('http://127.0.0.1:8086/api/register', {method: 'POST', headers: {"Content-Type": "application/json"}, body: JSON.stringify(this.registration_data)})
				.then((response) => response.json())
				.then((response) => {    
					console.log("Vue got Response");
					console.log("Response data: " + response);
					localStorage.setItem('user', JSON.stringify(response));
					this.$flashMessage.show({
						type: 'success',
						title: 'Successfully logged in',
						time: 1000
					});
					//this.$router.push({path: '/'});
				})
				.catch((errors) => {
					//console.log("Vue got Error");
					console.log("Error data: " + errors);
					this.$flashMessage.show({
						type: 'error',
						title: 'No good',
						time: 1000
					});
					//this.$router.push({path: '/'});
				})      
			}
		},
		mounted() {
			const route = useRoute()
			console.log(route.query)
			this.registration_data = route.query
			this.confirm_registration();
		}
	}
</script>