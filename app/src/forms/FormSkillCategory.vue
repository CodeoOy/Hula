<template>
	<v-form v-on:submit="createSkillCategory">
		<div class="mb-2">
			<label class="form-label">Category label</label>
			<error-message name="name" class="error"></error-message>
			<v-field
				v-model.number="queryData.label"
				:rules="isRequired"
				as="input"
				type="text"
				name="name"
				class="form-control"
				placeholder="Techs"
				aria-label="Category name"
			></v-field>
		</div>
		<div class="mb-2">
			<label class="form-label">Category parent (optional)</label>
			<error-message name="parent" class="error"></error-message>
			<v-field
				v-model="queryData.parent_id"
				:rules="isRequired"
				as="select"
				name="parent"
				class="form-select"
				aria-label="Category parent"
			>
				<option v-for="category in categories" :key="category" :value="category.id">
					{{ category.label }}
				</option>
			</v-field>
		</div>
		<button type="submit" class="btn btn-gradient">Submit</button>
	</v-form>   
</template>

<script>
import { Field, Form, ErrorMessage } from 'vee-validate';
export default {
	name: 'SkillCategory',
	data() {
		return {
			errorsPresent: false,
			categories: {},
			queryData: {
				email: this.$store.state.loggeduser.email,
				label: '',
				parent_id: null,
			}
		};
	},
	components: {
		'VForm': Form,
		'VField': Field,
		ErrorMessage
	},
	methods: {
		isRequired(value) {
			return value ? true : 'This field is required';
		},
		createSkillCategory() {
			fetch('/api/skills/categories', {
				method: 'POST',
				headers: {"Content-Type": "application/json"},
				credentials: 'include',
				body: JSON.stringify(this.queryData)
			})
			.then(() => {
				this.$router.go()
			})
			.catch((errors) => {
				console.log(errors);
			})
		},
		getSkillCategories() {
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