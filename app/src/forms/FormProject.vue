<template>
	<v-form v-on:submit="saveProject">
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
		<div class="mb-2 form-check" v-if="'is_hidden' in chosenProject">
			<label class="form-label">Hidden</label>
			<error-message name="category" class="error"></error-message>
			<input type="checkbox" class="form-check-input" name="is_hidden" v-model="formData.is_hidden" />
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
				id: this.chosenProject.id || undefined,
				name: this.chosenProject.name || '',
				is_hidden: this.chosenProject.is_hidden || false, // TODO: Does this work in both edit and new?
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
		async saveProject() {
			const project = await this.$api.saveProject(this.formData)
			if (project) this.$emit('formSent', project)
		},
	}
};
</script>