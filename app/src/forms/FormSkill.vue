<template>
	<v-form v-on:submit="createSkill">
		<div class="mb-2">
			<label class="form-label">Skill name</label>
			<error-message name="name" class="error"></error-message>
			<v-field
				v-model="formData.label"
				:rules="isRequired"
				as="input"
				type="text"
				name="name"
				class="form-control"
				aria-label="Skill name"
			></v-field>
		</div>
		<div class="mb-2" v-if="!('id' in chosenCategory) && categories.length">
			<label class="form-label">Skill category</label>
			<error-message name="category" class="error"></error-message>
			<v-field
				v-model="formData.skillcategory_id"
				:rules="isRequired"
				as="select"
				name="category"
				class="form-select"
				id="AddNewSkill"
				aria-label="Skill category"
			>
				<option v-for="category in categories" :key="category.label" :value="category.id">
					{{ category.label }}
				</option>
			</v-field>
		</div>
		<div class="mb-2" v-if="method == 'POST'">
			<label class="form-label">Skill scope</label>
			<error-message name="scope" class="error"></error-message>
			<v-field
				v-model="formData.skillscope_id"
				:rules="isRequired"
				as="select"
				name="scope"
				class="form-select"
				id="AddExistingSkillScope"
				aria-label="Pick scope"
			>
				<option v-for="scope in scopes" :key="scope.label" :value="scope.id">
					{{ scope.label }}
				</option>
			</v-field>
		</div>
		<button type="submit" class="btn btn-gradient mb-1">Submit</button>
	</v-form> 
</template>

<script>
import { Field, Form, ErrorMessage } from 'vee-validate';
export default {
	name: 'Skill',
	data() {
		return {
			categories: [],
			scopes: [],
			formData: {
				label: this.chosenSkill.label || '',
				skillcategory_id: this.chosenSkill.skillcategory_id || this.chosenCategory.id,
				skillscope_id: this.chosenSkill.skillscope_id || null,
			},
		}
	},
	components: {
		'VForm': Form,
		'VField': Field,
		ErrorMessage
	},
	props: {
		chosenSkill: {},
		chosenCategory: {},
		url: '',
		method: ''
	},	
	methods: {
		isRequired(value) {
			return value ? true : 'This field is required';
		},
		createSkill() {
			fetch(this.url, {
				method: this.method,
				headers: {"Content-Type": "application/json"},
				credentials: 'include',
				body: JSON.stringify(this.formData)
			})
			.then(response => {
				if (response.status >= 200 && response.status <= 299) {
					this.$emit('formSent')
				} else {
					this.$store.commit('errorHandling', response)
				}
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
		},
		getSkillScopes() {
			fetch('/api/skills/scopes', {method: 'GET'})
			.then(response => { 
				return (response.status >= 200 && response.status <= 299) ? response.json() : this.$store.commit('errorHandling', response)
			})
			.then(response => { 
				this.scopes = response;
				this.formData.skillscope_id = this.scopes[0].id
			})    
			.catch((errors) => {
				this.$store.commit('errorHandling', errors)
			})
		}
	},
	mounted() {
		this.getSkillCategories()
		this.getSkillScopes()
	},
};
</script>