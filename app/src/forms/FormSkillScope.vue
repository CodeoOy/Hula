<template>
	<v-form v-on:submit="createUpdateSkillScope">
		<div class="mb-2">
			<label class="form-label">Scope name</label>
			<error-message name="name" class="error"></error-message>
			<v-field
				name="name" 
				type="text" 
				placeholder="One to ten" 
				:rules="isRequired" 
				class="form-control" 
				v-model="formData.label"
			></v-field>
		</div>
		<button type="submit" class="btn btn-gradient mb-1">Submit</button>
	</v-form>  
</template>

<script>
import { Field, Form, ErrorMessage } from 'vee-validate';
export default {
	name: 'SkillScope',
	data() {
		return {
			categories: {},
			formData: {
				label: this.chosenScope.label || ''
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
		url: '',
		method: ''
	},	
	methods: {
		isRequired(value) {
			return value ? true : 'This field is required';
		},
		createUpdateSkillScope() {
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
		}
	}
};
</script>