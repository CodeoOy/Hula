<template>
	<v-form v-on:submit="createSkill">
		<div class="mb-2">
			<error-message name="name" class="error"></error-message>
			<label class="form-label">Skill name</label>
			<v-field
				v-model="chosenSkill.label"
				:rules="isRequired"
				as="input"
				type="text"
				name="name"
				class="form-control"
				aria-label="Skill name"
			></v-field>
		</div>
		<div class="mb-2">
			<label class="form-label">Skill category</label>
			<error-message name="category" class="error"></error-message>
			<v-field
				v-model="chosenSkill.skillcategory_id"
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
				v-model="chosenSkill.skillscope_id"
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
		url: '',
		method: ''
	},	
	methods: {
		isRequired(value) {
			return value ? true : 'This field is required';
		},
		createSkill() {
			this.chosenSkill.email = this.$store.state.loggeduser.email,
			fetch(this.url, {
				method: this.method,
				headers: {"Content-Type": "application/json"},
				credentials: 'include',
				body: JSON.stringify(this.chosenSkill)
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
		},
		getSkillScopes() {
			fetch('/api/skills/scopes', {method: 'GET'})
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
	},
};
</script>