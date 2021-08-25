<template>
	<div>
		<h2 v-if="formData.projectname.length">{{ formData.projectname }}</h2>
		<VAutoComplete
			v-if="users.length"
			:suggestions="users"
			:placeholder="'Start typing the name of the pro..'" 
			:dropdown="true"
			dropdownLabel="firstname"
			:filterProperties="'firstname'"
			:selection="value"
			v-on:auto-complete="matchedProjects"
		></VAutoComplete>
	</div>
</template>

<script>
	import VAutoComplete from '../components/VAutoComplete.vue'
	export default {
		name: 'FindLead',
		data() {
			return {
				value: '',
				users: [],
				selected: {},
				formData: {
					projectname: '',
				},
			}
		},
		components: {
			VAutoComplete
		},
		methods: {
			matchedProjects(value) {
				// get the projects that have user's user_id in matches
				let projects = this.$store.state.projects.filter(project => {
					return project.matches.some(match => {
						console.log(`p: ${project.name} | m: ${match.user_id} | u: ${value.id}`)
						return match.user_id === value.id
					})
				})
				this.$emit('leadsfetched', projects)
			},
			allUsers() {
				fetch('/api/users', {method: 'GET'})
				.then(response => { 
					return (response.status >= 200 && response.status <= 299) ? response.json() : this.$store.commit('errorHandling', response)
				})
				.then(response => {
					this.users = response
				})
				.catch((errors) => {
					this.$store.commit('errorHandling', errors)
				})
			},
		},
		mounted() {
			this.allUsers();
			if (!this.$store.state.projects.length) this.$store.dispatch('getProjects')	
		}
	}
</script>
