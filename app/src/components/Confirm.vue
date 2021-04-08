<template>
	<div class="container">
		{{ registration_data }}
	</div>
</template>

<script>
	import { useRoute } from 'vue-router'
	export default {
		name: 'Cheeseboi',
		data: function() {
			return {
				message: JSON,
				registration_data: {},
				raw: {
					email: "tuoppil@gmail.com",
					password: "salasana",
					id: "e01f7982-1275-4b3f-bea5-2aa3dff59949"
				}
			};
		},
		methods: {
			confirm_registration: function() {  
				fetch(`http://127.0.0.1:8086/api/register/${this.registration_data.id}`, {method: 'POST', headers: {"Content-Type": "application/json"}, body: JSON.stringify(this.registration_data)})
				.then((response) => response.json())
				.then((response) => {    
					console.log("Vue got Response");
					console.log("Response data: " + response);
					//localStorage.setItem('user', JSON.stringify(response));
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
			this.registration_data = route.query
			console.log(route.query)
			console.log(this.registration_data)
			console.log(this.raw)
			this.confirm_registration();
		}
	}
</script>