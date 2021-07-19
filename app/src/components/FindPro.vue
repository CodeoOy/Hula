<template>
	<div>
		<form action="#" v-on:submit.prevent="getMatchedUsers">
			<select class="mb-2 form-select" v-model="queryData.projectname" aria-label="Choose project">
				<option :value="''" disabled>Choose the project</option>
				<option v-for="project in projects" :key="project.name" :value="project.name">{{ project.name }}</option>
			</select>
			<button type="submit" class="btn btn-gradient mb-1">Search</button>
		</form>
		<p>Form below is a demo of autocomplete. It doesn't do anything yet.</p>
		<AutoComplete v-if="this.$store.state.projects.length" :suggestions="this.$store.state.projects" :selection.sync="value"></AutoComplete>
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
			getMatchedUsers() {
				fetch('api/matchedusers', {
					method: 'POST',
					headers: {"Content-Type": "application/json"},
					credentials: 'include',
					body: JSON.stringify(this.queryData)
				})
				.then(response => {
					this.$store.commit('errorHandling', response)
				})  
				.then(response => { 
					return (response.status >= 200 && response.status <= 299) ? response.json() : this.$store.commit('errorHandling', response)
				})
				.then(response => {
					this.users = response;
					this.$emit('usersfetched', this.users)
					console.log("Got to the fetching")
				})    
				.catch((errors) => {
					console.log(errors)
				})
			}
		}
	}
</script>