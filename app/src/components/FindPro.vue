<template>
	<div>
		<h2 v-if="queryData.projectname.length">{{ queryData.projectname }}</h2>
		<AutoComplete 
			v-if="this.$store.state.projects" 
			:suggestions="this.$store.state.projects"
			:placeholder="'Start typing the name of project..'" 
			:dropdown="true"
			:filterProperties="'name'"
			:selection.sync="value"
			v-on:auto-complete="getMatchedUsers($event)"
		></AutoComplete>
	</div>
</template>

<script>
	import AutoComplete from '../components/AutoComplete.vue'
	export default {
		name: 'FindPro',
		data() {
			return {
				value: '',
				projects: this.$store.state.projects,
				user: {},
				users: {},
				selected: {},
				queryData: {
					projectname: '',
				},
			}
		},
		components: {
			AutoComplete
		},
		methods: {
			getMatchedUsers(value) {
				this.users = value.matches
				this.$emit('usersfetched', this.users)
			}
		}
	}
</script>