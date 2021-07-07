<template>
	<v-form v-on:submit="createUpdateSkillCategory">
		<div class="mb-2">
			<label class="form-label">Category label</label>
			<error-message name="name" class="error"></error-message>
			<v-field
				:rules="isRequired"
				as="input"
				type="text"
				name="name"
				class="form-control"
				placeholder="Techs"
				aria-label="Category name"
				v-model="queryData.label"
			></v-field>
		</div>
		<div class="mb-2">
			<label class="form-label">Category parent (optional)</label>
			<error-message name="parent" class="error"></error-message>
			<v-field
				as="select"
				name="parent"
				class="form-select"
				aria-label="Category parent"
				v-model="queryData.parent_id"
			>
				<option :value="''">No parent</option>
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
			categories: {},
		};
	},
	components: {
		'VForm': Form,
		'VField': Field,
		ErrorMessage
	},
	props: {
		chosenCategory: {},
		url: '',
		method: '',
	},
	methods: {
		isRequired(value) {
			return value ? true : 'This field is required';
		},
		createUpdateSkillCategory() {
			if (!this.queryData.parent_id.length) {
				delete this.queryData.parent_id
			}
			fetch(this.url, {
				method: this.method,
				headers: {"Content-Type": "application/json"},
				credentials: 'include',
				body: JSON.stringify(this.queryData)
			})
			.then(() => {
				this.$emit('formSent')
			})
			.catch((errors) => {
				this.$store.commit('errorHandling', errors)
			})
		},
		getSkillCategories() {
			fetch('/api/skills/categories', {method: 'GET'})
			.then(response => { 
				return (response.status >= 200 && response.status <= 299) ? response.json() : this.$store.commit('errorHandling', response)
			})
			.then(response => { 
				this.categories = response;
			})    
			.catch((errors) => {
				this.$store.commit('errorHandling', errors)
			})
		}
	},
	mounted() {
		this.getSkillCategories()
	},
	computed: {
		queryData: {
			get () {
				if (this.chosenCategory) {
					return this.chosenCategory
				}
				console.log("No chosen category")
				return {
					email: "",
					label: "",
					parent_id: null,
				}
			}
		}
	}
};
</script>