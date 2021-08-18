<template>
	<div>
		<h2 v-if="queryData.projectname.length">{{ queryData.projectname }}</h2>
		<VAutoComplete 
			v-if="this.$store.state.projects" 
			:suggestions="this.$store.state.projects"
			:placeholder="'Start typing the name of project..'" 
			:dropdown="true"
			:filterProperties="'name'"
			:selection="value"
			v-on:auto-complete="getChosenProject($event)"
		></VAutoComplete>
	</div>
</template>

<script>
	import VAutoComplete from '../components/VAutoComplete.vue'
	export default {
		name: 'FindPro',
		data() {
			return {
				value: '',
				projects: this.$store.state.projects,
				user: {},
				selected: {},
				queryData: {
					projectname: '',
				},
			}
		},
		components: {
			VAutoComplete
		},
		methods: {
			getChosenProject(value) {
				fetch(`/api/matches/${value.id}`, {method: 'GET'})
				.then(response => { 
					return (response.status >= 200 && response.status <= 299) ? response.json() : this.$store.commit('errorHandling', response)
				})
				.then(response => {
					this.$emit('matchesfetched', response)
				})    
				.catch((errors) => {
					this.$store.commit('errorHandling', errors)
				})				
			},
		}
	}
</script>
