<template>
	<form v-on:submit="createProject">
		<p v-if="errorsPresent" class="error">Please fill out label!</p>
		<div class="mb-2">
			<label class="form-label">Project name</label>
			<input class="form-control" type="text" placeholder="Languages" name="label" v-model="querydata.name" />
		</div>
		<button type="submit" class="btn btn-gradient mb-1">Submit</button>
	</form> 
</template>

<script>
export default {
	name: 'Project',
	data() {
		return {
			querydata: {
				name: '',
			}	
		}
	},
	methods: {
		createProject: function() {
			fetch('/api/project', {
				method: 'POST',
				headers: {"Content-Type": "application/json"},
				credentials: 'include',
				body: JSON.stringify(this.querydata)
			})
		}
	},
};
</script>