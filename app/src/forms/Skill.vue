<template>
	<form v-on:submit="createSkill">
		<p v-if="errorsPresent" class="error">Please fill out label!</p>
		<div class="mb-2">
			<label class="form-label">Skill name</label>
			<input class="form-control" type="text" placeholder="Languages" name="label" v-model="querydata.label" />
		</div>
		<div class="mb-2">
			<label class="form-label">Skill category</label>
			<select class="form-select mb-2" id="Skill" aria-label="Skill category" v-model="querydata.category_id">
				<option v-for="category in categories" :key="category" :value="category.id">
					{{ category.label }}
				</option>
			</select>
		</div>
		<div class="mb-2">
			<label class="form-label">Skill scope</label>
			<select class="form-select mb-2" id="Skill" aria-label="Skill scope" v-model="querydata.skillscope_id">
				<option v-for="scope in scopes" :key="scope" :value="scope.id">
					{{ scope.label }}
				</option>
			</select>
		</div>
		<button type="submit" class="btn btn-gradient mb-1">Submit</button>
	</form> 
</template>

<script>
export default {
	name: 'Skill',
	data() {
		return {
			querydata: {
				label: '',
				category_id: null,
				skillscope_id: null,
				email: this.$store.state.loggeduser.email,
			},
			categories: {},
			scopes: {},		
		}
	},
	methods: {
		createSkill: function() {
			fetch('http://localhost:8086/api/skill', {
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
		getSkillScopes: function() {
			fetch('http://localhost:8086/api/skills/scopes', {method: 'GET'})
			.then((response) => response.json())
			.then(response => { 
				this.scopes = response;
			})    
			.catch((errors) => {
				console.log(errors);
			})
		}
	},
	mounted() {
		this.getSkillCategories()
		this.getSkillScopes()
	}
};
</script>