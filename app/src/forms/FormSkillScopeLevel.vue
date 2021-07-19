<template>
	<v-form v-on:submit="createUpdateSkillScopeLevel">
		<div class="mb-2">
			<label class="form-label">Level name</label>
			<error-message name="name" class="error"></error-message>
			<v-field
				v-model="formData.label"
				:rules="isRequired"
				as="input"
				type="text"
				name="name"
				class="form-control"
				placeholder="Rookie"
				aria-label="Level name"
			></v-field>
		</div>
		<div class="mb-2">
			<label class="form-label">Percentage</label>
			<error-message name="percentage" class="error"></error-message>
			<v-field
				v-model.number="formData.percentage"
				:rules="isRequired"
				as="input"
				type="number"
				name="percentage"
				class="form-control"
				aria-label="Level percentage"
			></v-field>
		</div>
		<button type="submit" class="btn btn-gradient mb-1">Submit</button>
	</v-form> 
</template>

<script>
import { Field, Form, ErrorMessage } from 'vee-validate';
export default {
	name: 'SkillScopeLevel',
	data() {
		return {
			scopes: {},
			formData: {
				label: this.chosenLevel.label || '',
				percentage: this.chosenLevel.percentage || 0,
				skillscope_id: this.chosenScope.id,
				email: '',
			}
		};
	},
	components: {
		'VForm': Form,
		'VField': Field,
		ErrorMessage
	},
	props: {
		chosenScope: {},
		chosenLevel: {},
		url: '',
		method: ''
	},	
	methods: {
		isRequired(value) {
			return value ? true : 'This field is required';
		},
		createUpdateSkillScopeLevel() {
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
		getSkillScopes() {
			fetch('/api/skills/scopes', {method: 'GET'})
			.then(response => { 
				return (response.status >= 200 && response.status <= 299) ? response.json() : this.$store.commit('errorHandling', response)
			})
			.then(response => { 
				this.scopes = response;
			})    
			.catch((errors) => {
				this.$store.commit('errorHandling', errors)
			})
		}
	},
	mounted() {
		this.getSkillScopes()
		console.log(this.chosenScope)
	}
};
</script>