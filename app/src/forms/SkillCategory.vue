<template>
	<form action="#" @submit.prevent="onSubmit">
		<p v-if="errorsPresent" class="error">Please fill out label!</p>
		<div class="mb-2">
			<label class="form-label">Category label</label>
			<input class="form-control" type="text" placeholder="Languages" name="label" v-model="querydata.label" />
		</div>
		<div class="mb-2">
			<select class="form-select" id="SkillParent" aria-label="Example select with button addon" v-model="querydata.parent_id">
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
	props: {	
		user: {}
	},
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
		onSubmit: function() {
			if (this.querydata.label === '') {
				this.errorsPresent = true;
				this.$flashMessage.show({
					type: 'error',
					title: 'Please fill out label.',
					time: 1000
				});
			} else {
				//this.$emit('formsent', this.user);
				fetch('http://localhost:8086/api/skills/category', {
					method: 'POST',
					headers: {"Content-Type": "application/json"},
					credentials: 'include',
					body: JSON.stringify(this.querydata)
				})
			}
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