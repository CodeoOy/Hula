<template>
	<div>
		<h2 v-if="formData.projectname.length">{{ formData.projectname }}</h2>
		<VAutoComplete
			v-if="users.length"
			:suggestions="users"
			placeholder='Pro'
			:dropdown="true"
			dropdownLabel="firstname"
			:filterProperties="'firstname'"
			v-on:auto-complete="getMatches"
		></VAutoComplete>
	</div>
</template>

<script>
	import VAutoComplete from '../components/VAutoComplete.vue'
	export default {
		name: 'FindLead',
		data() {
			return {
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
			getMatches(user) {
				// get the projects that have user's user_id in matches
				let projects = this.$store.state.projects.filter(project => {
					return project.matches.some(match => {
						return match.user_id === user.id
					})
				})
				this.$emit('leadsfetched', projects)
			},
		},
		async mounted() {
			if (!this.$store.state.projects.length) this.$store.dispatch('getProjects')	
			this.users = await this.$api.users.get()
		}
	}
</script>
