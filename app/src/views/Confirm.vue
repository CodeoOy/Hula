<template>
	<div class="container">
		<div class="row mt-4">
			<div class="col">
				<div class="content-box rounded-2 p-3">
					<a href="#" v-on:click="getRustData">Test link. May produce a treefloof.</a>
					<p v-if="message.length">{{ message }}</p>
					<Confirm />
				</div>
			</div>
		</div>
	</div>
</template>

<script>
	import Confirm from '../components/Confirm.vue'
	export default {
		name: 'Confirm',
		data() {
			return {
				message: JSON,
				registration_data: String,
			}
		},
		components: {
			Confirm: Confirm
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
			}
		}
	}
</script>