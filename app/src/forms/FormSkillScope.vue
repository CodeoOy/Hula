<template>
	<v-form v-on:submit="createUpdateSkillScope">
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
			queryData: {
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
				body: JSON.stringify(this.queryData)
			})
			.then(() => {
				this.$emit('formSent')
			})
			.catch((errors) => {
				console.log(errors);
			})
		}
	}
};
</script>