<template>
	<div class="p-3 rounded-2 content-box bg-dark text-light">
		<h2>Raw registration data:</h2>
		<pre>{{ registration_data }}</pre>
	</div>
</template>

<script>
	import { useRoute } from 'vue-router'
	export default {
		name: 'ConfirmUser',
		data: function() {
			return {
				registration_data: {},
			};
		},
		methods: {
			confirm_registration: function() {  
				fetch(`/api/register/${this.registration_data.id}`, {
					method: 'POST', 
					headers: {"Content-Type": "application/json"}, 
					body: JSON.stringify(this.registration_data)
				})
				.then((response) => response.json())
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
			this.registration_data = route.query
			this.confirm_registration();
		}
	}
</script>