<template>
	<form v-on:submit="createSkillScope">
		<p v-if="errorsPresent" class="error">Please fill out label!</p>
		<div class="mb-2">
			<label class="form-label">Scope name</label>
			<input class="form-control" type="text" placeholder="Language levels" name="scopename" v-model="querydata.label" />
		</div>
		<div class="mb-2">
			<label class="form-label">Scope category</label>
			<select class="form-select mb-2" id="SkillScope" aria-label="Scope category" v-model="querydata.category_id">
				<option v-for="category in categories" :key="category" :value="category.id">
					{{ category.label }}
				</option>
			</select>
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
			querydata: {
				email: this.$store.state.loggeduser.email,
				label: "",
				category_id: null,
			},
			categories: {},
		};
	},
	methods: {
		createSkillScope: function() {
			fetch('/api/skills/scopes', {
				method: 'POST',
				headers: {"Content-Type": "application/json"},
				credentials: 'include',
				body: JSON.stringify(this.querydata)
			})
			.catch((errors) => {
				console.log(errors);
			})
		},
		getSkillCategories: function() {
			fetch('/api/skills/categories', {method: 'GET'})
			.then((response) => response.json())
			.then(response => { 
				this.categories = response;
			})    
			.catch((errors) => {
				console.log(errors);
			})
		},
	},
	mounted() {
		this.getSkillCategories()
	}
};
</script>