<template>
	<v-form v-on:submit="createUpdateProject">
		<div class="mb-2">
			<label class="form-label">Project name</label>
			<error-message name="name" class="error"></error-message>
			<v-field
				v-model="formData.name"
				:rules="isRequired"
				as="input"
				type="text"
				name="name"
				class="form-control"
				aria-label="Project name"
			></v-field>
		</div>
		<div class="mb-2">
			<label class="form-label">Is the project available?</label>
			<error-message name="category" class="error"></error-message>
			<v-field
				v-model="formData.available"
				:rules="isRequired"
				as="checkbox"
				name="available"
				class="form-checkbox"
				aria-label="Skill category"
			></v-field>
		</div>
		<button type="submit" class="btn btn-gradient mb-1">Submit</button>
	</v-form> 
</template>

<script>
import { Field, Form, ErrorMessage } from 'vee-validate';
export default {
	name: 'FormProject',
	data() {
		return {
			formData: {
				name: this.chosenProject.name || '',
				available: this.chosenProject.available || true,
			},
		}
	},
	components: {
		'VForm': Form,
		'VField': Field,
		ErrorMessage
	},
	props: {
		chosenProject: {},
		url: '',
		method: ''
	},	
	methods: {
		isRequired(value) {
			return value ? true : 'This field is required';
		},
		createUpdateProject() {
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
	},
};
</script>