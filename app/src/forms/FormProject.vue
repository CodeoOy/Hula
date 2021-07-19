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
		<div class="mb-2 form-check" v-if="'available' in chosenProject">
			<label class="form-label">Published and visible</label>
			<error-message name="category" class="error"></error-message>
			<input type="checkbox" class="form-check-input" name="available" v-model="formData.available" />
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
				available: this.chosenProject.available,
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
	}
};
</script>