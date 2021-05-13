<template>
	<div>
		<form action="#" @submit.prevent="onSubmit" v-on:submit="getMatchedUsers">
			<select class="mb-2 form-select" v-model="querydata.projectname" aria-label="Choose project">
				<option disabled>Choose the project</option>
				<option v-for="project in projects" :key="project.name" :value="project.name">{{ project.name }}</option>
			</select>
			<!--<div class="mb-2 form-check">
				<label class="form-label">Show only available candidates</label>
				<input type="checkbox" class="form-check-input" name="available" v-model="user.available" />
			</div>-->
			<button type="submit" class="btn btn-gradient mb-1">Search</button>
		</form>
	</div>
</template>

<script>
	export default {
		name: 'FindPro',
		data() {
			return {
				projects: {},
				user: {},
				users: {},
				selected: {},
				querydata: {
					projectname: '',
				},
			}
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
			getUserData: function(uid) { 
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
			getUsers: function() {
				fetch('api/users', {method: 'GET'})
				.then((response) => response.json())
				.then(response => { 
					this.users = response;
					this.$emit('usersfetched', this.users)
				})    
				.catch((errors) => {
					console.log(errors);
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
					console.log(errors);
				})
			}
		},
		mounted() {
			this.getProjects()
		}
	}
</script>