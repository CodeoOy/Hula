<template>
	<div>
		<form action="#" v-on:submit.prevent="getMatchedUsers">
			<select class="mb-2 form-select" v-model="queryData.projectName" aria-label="Choose project">
				<option :value="''" disabled>Choose the project</option>
				<option v-for="project in projects" :key="project.name" :value="project.name">{{ project.name }}</option>
			</select>
			<button type="submit" class="btn btn-gradient mb-1">Search</button>
		</form>
		<p>Form below is a demo of autocomplete. It doesn't do anything yet.</p>
		<AutoComplete :suggestions="testprojects" :selection.sync="value"></AutoComplete>
	</div>
</template>

<script>
	import AutoComplete from '../components/AutoComplete.vue'
	export default {
		name: 'FindPro',
		data() {
			return {
				testprojects: [
					"mansikka",
					"mustikka",
					"vadelma"
				],
				value: '',
				projects: this.$store.state.projects,
				user: {},
				users: {},
				selected: {},
				queryData: {
					projectName: '',
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
				.then((response) => response.json())
				.then(response => { 
					this.users = response;
					this.$emit('usersfetched', this.users)
				})    
				.catch((errors) => {
					this.users = {}
					console.log(errors);
				})
			}
		}
	}
</script>