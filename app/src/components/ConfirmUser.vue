<template>
	<div class="p-3 rounded-2 content-box bg-dark text-light">
		<h2>Raw registration data:</h2>
		<pre>{{ registrationData }}</pre>
	</div>
</template>

<script>
	import { useRoute } from 'vue-router'
	export default {
		name: 'ConfirmUser',
		data: function() {
			return {
				registrationData: {},
			};
		},
		methods: {
			confirmRegistration: function() {  
				fetch(`/api/register/${this.registrationData.id}`, {
					method: 'POST', 
					headers: {"Content-Type": "application/json"}, 
					body: JSON.stringify(this.registrationData)
				})
				.then(response => { 
					return (response.status >= 200 && response.status <= 299) ? response.json() : this.$store.commit('errorHandling', response)
				})
				.then(() => {    
					this.$flashMessage.show({
						type: 'success',
						title: 'Account confirmed.',
						time: 2000
					});
				})
				.catch((errors) => {
					console.log("Error data: " + errors);
					this.$flashMessage.show({
						type: 'error',
						title: 'Account not confirmed',
						time: 2000
					});
				})      
			}
		},
		mounted() {
			const route = useRoute()
			this.registrationData = route.query
			this.confirmRegistration();
		}
	}
</script>