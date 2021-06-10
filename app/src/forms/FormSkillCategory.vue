<template>
	<form v-on:submit="createSkillCategory">
		<p v-if="errorsPresent" class="error">Please fill out label!</p>
		<div class="mb-2">
			<label class="form-label">Category label</label>
			<input class="form-control" type="text" placeholder="Techs" name="categorylabel" v-model="querydata.label" />
		</div>
		<div class="mb-2">
			<label class="form-label">Category parent (optional)</label>
			<select class="form-select" id="SkillParent" aria-label="Skill category parent" v-model="querydata.parent_id">
				<option v-for="category in categories" :key="category" :value="category.id">
					{{ category.label }}
				</option>
			</select>
		</div>
		<button type="submit" class="btn btn-gradient">Submit</button>
	</form>   
</template>

<script>
export default {
	name: 'SkillCategory',
	data() {
		return {
			errorsPresent: false,
			categories: {},
			querydata: {
				email: this.$store.state.loggeduser.email,
				label: '',
				parent_id: null,
			}
		};
	},
	methods: {
		createSkillCategory: function() {
			if (this.querydata.label === '') {
				this.errorsPresent = true;
			} else {
				fetch('/api/skills/categories', {
					method: 'POST',
					headers: {"Content-Type": "application/json"},
					credentials: 'include',
					body: JSON.stringify(this.querydata)
				})
			}
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
		}
	},
	mounted() {
		this.getSkillCategories()
	}
};
</script>