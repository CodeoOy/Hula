<template>
	<form v-on:submit="createSkill">
		<div class="input-group">
			<input type="text" aria-label="Label" class="form-control" v-model="skilldata.label" >
			<select class="form-select" id="skillCategoryId" aria-label="Example select with button addon" v-model="skilldata.category_id">
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
	name: 'Skill',
	data() {
		return {
			skilldata: {
				label: '',
				category_id: '',
				email: this.$store.state.loggeduser.email,
			},
			categories: {}			
		}
	},
	methods: {
		createSkill: function() {
			fetch('http://localhost:8086/api/skill', {
				method: 'POST',
				headers: {"Content-Type": "application/json"},
				credentials: 'include',
				body: JSON.stringify(this.skilldata)
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
		}
	},
	mounted() {
		this.getSkillCategories()
	}
};
</script>