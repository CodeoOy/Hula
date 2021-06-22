<template>
	<v-form v-on:submit="createSkillScope">
		<error-message name="name" class="error"></error-message>
		<div class="mb-2">
			<label class="form-label">Scope name</label>
			<v-field name="name" type="text" placeholder="One to ten" :rules="isRequired" class="form-control" v-model="queryData.label"></v-field>
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
			errorsPresent: false,
			queryData: {
				email: this.$store.state.loggeduser.email,
				label: "",
			},
			categories: {},
		};
	},
	components: {
		'VForm': Form,
		'VField': Field,
		ErrorMessage
	},
	methods: {
		isRequired(value) {
			return value ? true : 'This field is required';
		},
		createSkillScope() {
			fetch('/api/skills/scopes', {
				method: 'POST',
				headers: {"Content-Type": "application/json"},
				credentials: 'include',
				body: JSON.stringify(this.queryData)
			})
			.catch((errors) => {
				console.log(errors);
			})
		}
	}
};
</script>