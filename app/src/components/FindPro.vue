<template>
	<div>
		<h2 v-if="formData.projectname.length">{{ formData.projectname }}</h2>
		<VAutoComplete 
			v-if="this.$store.state.projects" 
			:suggestions="this.$store.state.projects"
			placeholder='Project'
			:dropdown="true"
			:filterProperties="'name'"
			v-on:auto-complete="getMatches"
		></VAutoComplete>
	</div>
</template>

<script>
	import VAutoComplete from '../components/VAutoComplete.vue'
	export default {
		name: 'FindPro',
		data() {
			return {
				projects: this.$store.state.projects,
				selected: {},
				formData: {
					projectname: '',
				},
			}
		},
		components: {
			VAutoComplete
		},
		mounted() {
			if (!this.$store.state.projects.length) this.$store.dispatch('getProjects')	
		},
		methods: {
			async getMatches(project) {
				if (!project) return null
				const matches = await this.$api.matches.get(project.id)
				if (matches) this.$emit('matchesfetched', { project, matches })
			},
		}
	}
</script>
