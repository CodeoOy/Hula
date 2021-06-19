<template>
	<div>
		<form action="#" v-on:submit.prevent="getMatchedUsers">
			<select class="mb-2 form-select" v-model="querydata.projectname" aria-label="Choose project">
				<option disabled>Choose the project</option>
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
				projects: {},
				user: {},
				users: {},
				selected: {},
				querydata: {
					projectname: '',
				},
			}
		},
		components: {
			AutoComplete
		},
		methods: {
			getProjects: function() {
				fetch('api/projects', {method: 'GET'})
				.then((response) => response.json())
				.then(response => { 
					this.projects = response;
					this.$emit('datafetched', this.projects)
				})    
				.catch((errors) => {
					console.log(errors);
				})
			},
			getProjectData: function(pid) { 
				fetch('api/project', {
					method: 'POST',
					headers: {"Content-Type": "application/json"},
					body: JSON.stringify({"pid": pid})
				})
				.then((response) => response.json())
				.then(response => {
					this.project = response;
				})    
			},
			getUserData: function(uid) { // Currently not used but there should be a modal with this data
				fetch('api/user', {
					method: 'POST',
					headers: {"Content-Type": "application/json"},
					body: JSON.stringify({"uid": uid})
				})
				.then((response) => response.json())
				.then(response => { 
					this.user = response;
				})    
			},
			getMatchedUsers: function() {
				fetch('api/matchedusers', {
					method: 'POST',
					headers: {"Content-Type": "application/json"},
					credentials: 'include',
					body: JSON.stringify(this.querydata)
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
		},
		mounted() {
			this.getProjects()
		}
	}
</script>