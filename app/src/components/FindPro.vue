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
				console.log(value)
				if (value) {
					this.queryData.projectname = value.name
				}
				fetch('api/matchedusers', {
					method: 'POST',
					headers: {"Content-Type": "application/json"},
					credentials: 'include',
					body: JSON.stringify(this.queryData)
				}) 
				.then(response => { 
					return (response.status >= 200 && response.status <= 299) ? response.json() : this.$store.commit('errorHandling', response)
				})
				.then(response => {
					this.users = response;
					this.$emit('usersfetched', this.users)
				})    
				.catch((errors) => {
					console.log(errors)
					this.users = {}
					this.$emit('usersfetched', this.users)
				})
			}
		}
	}
</script>