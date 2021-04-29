<template>
	<div class="container">
		{{ registration_data }}
	</div>
</template>

<script>
	import { useRoute } from 'vue-router'
	export default {
		name: 'Confirm',
		data: function() {
			return {
				registration_data: {},
			};
		},
		methods: {
			confirm_registration: function() {  
				fetch(`http://localhost:8086/api/register/${this.registration_data.id}`, {method: 'POST', headers: {"Content-Type": "application/json"}, body: JSON.stringify(this.registration_data)})
				.then((response) => response.json())
				.then((response) => {    
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