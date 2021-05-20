<template>
	<form action="#" @submit.prevent="createSkillScope">
		<div class="mb-2">
			<label class="form-label">Scope name</label>
			<input class="form-control" type="text" placeholder="Languages" name="label" v-model="querydata.label" />
		</div>
		<select class="form-select mb-2" id="Skill" aria-label="Example select with button addon" v-model="querydata.category_id">
			<option v-for="category in categories" :key="category" :value="category.id">
				{{ category.label }}
			</option>
		</select>
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
			fetch('http://localhost:8086/api/skills/scope', {
				method: 'POST',
				headers: {"Content-Type": "application/json"},
				credentials: 'include',
				body: JSON.stringify(this.querydata)
			})
		},
		getSkillCategories: function() {
			fetch('http://localhost:8086/api/skills/categories', {method: 'GET'})
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