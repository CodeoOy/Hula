<template>
	<div class="container">
		{{ registration_data }}
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
			confirm_registration: function() {  
				fetch('/api/register', {method: 'POST', headers: {"Content-Type": "application/json"}, body: JSON.stringify(this.registration_data.id)})
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
			console.log(this.registration_data.id)
			this.confirm_registration();
		}
	}
</script>