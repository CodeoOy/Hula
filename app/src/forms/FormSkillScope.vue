<template>
	<v-form v-on:submit="saveSkillScope">
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
				id: this.chosenScope.id || undefined,
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
		async saveSkillScope() {
			const scope = await this.$api.skills.scopes.save(this.formData)
			if (scope) this.$emit('formSent')
		}
	}
};
</script>