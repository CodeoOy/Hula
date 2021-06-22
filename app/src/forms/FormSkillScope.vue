<template>
	<form v-on:submit="createSkillScope">
		<p v-if="errorsPresent" class="error">Please fill out label!</p>
		<div class="mb-2">
			<label class="form-label">Scope name</label>
			<input class="form-control" type="text" placeholder="Language levels" name="scopename" v-model="queryData.label" />
		</div>
		<button type="submit" class="btn btn-gradient mb-1">Submit</button>
	</form>  
</template>

<script>
export default {
	name: 'SkillScope',
	data() {
		return {
			errorsPresent: false,
			queryData: {
				email: this.$store.state.loggeduser.email,
				label: "",
			},
			categories: {},
		};
	},
	methods: {
		createSkillScope() {
			fetch('/api/skills/scopes', {
				method: 'POST',
				headers: {"Content-Type": "application/json"},
				credentials: 'include',
				body: JSON.stringify(this.queryData)
			})
			.catch((errors) => {
				console.log(errors);
			})
		}
	}
};
</script>