<template>
	<v-form v-on:submit="createSkill">
		<div class="mb-2">
			<error-message name="name" class="error"></error-message>
			<label class="form-label">Skill name</label>
			<v-field
				v-model="queryData.label"
				:rules="isRequired"
				as="input"
				type="text"
				name="name"
				class="form-control"
				aria-label="Skill name"
			></v-field>
		</div>
		<div class="mb-2" v-if="chosenSkill">
			<label class="form-label">Skill category</label>
			<error-message name="category" class="error"></error-message>
			<v-field
				v-model="queryData.skillcategory_id"
				:rules="isRequired"
				as="select"
				name="category"
				class="form-select"
				id="AddNewSkill"
				aria-label="Skill category"
			>
				<option v-for="category in categories" :key="category" :value="category.id">
					{{ category.label }}
				</option>
			</v-field>
		</div>
		<div class="mb-2" v-if="method == 'POST'">
			<label class="form-label">Skill scope</label>
			<error-message name="scope" class="error"></error-message>
			<v-field
				v-model="queryData.skillscope_id"
				:rules="isRequired"
				as="select"
				name="scope"
				class="form-select"
				id="AddExistingSkillScope"
				aria-label="Pick scope"
			>
				<option v-for="scope in scopes" :key="scope" :value="scope.id">
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
			categories: {},
			scopes: {},
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
			delete this.queryData.id
			delete this.queryData.updated_by
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
			.then((response) => response.json())
			.then(response => { 
				this.categories = response;
			})    
			.catch((errors) => {
				this.$store.commit('errorHandling', errors)
			})
		},
		getSkillScopes() {
			fetch('/api/skills/scopes', {method: 'GET'})
			.then((response) => response.json())
			.then(response => { 
				this.scopes = response;
			})    
			.catch((errors) => {
				this.$store.commit('errorHandling', errors)
			})
		}
	},
	mounted() {
		console.log(this.queryData)
		this.getSkillCategories()
		this.getSkillScopes()
	},
	computed: {
		queryData: {
			get () {
				if (this.chosenSkill) {
					return {
						updated_by: "",
						id: this.chosenSkill.id,
						skillscope_id: this.chosenSkill.skillscope_id,
						skillcategory_id: this.chosenSkill.skillcategory_id,
						label: this.chosenSkill.label,
					}
				} else if (this.chosenCategory){
					return {
						updated_by: "",
						id: "",
						skillscope_id: "",
						skillcategory_id: this.chosenCategory.id,
						label: "",
					}
				} else {
					return {
						updated_by: "",
						id: this.chosenSkill.id,
						skillscope_id: this.chosenSkill.skillscope_id,
						skillcategory_id: this.chosenSkill.skillcategory_id,
						label: "",
					}
				}
			}
		}
	},
};
</script>