<template>
	<v-form v-on:submit="onSubmit">
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
				aria-label="Label"
			></v-field>
		</div>
		<button type="submit" class="btn btn-gradient mb-1">Submit</button>
	</v-form> 
</template>

<script>
import { Field, Form, ErrorMessage } from 'vee-validate';
export default {
	name: 'FormGeneralRename',
	data() {
		return {
			queryData: {
				label: '',
				email: this.$store.state.loggeduser.email,
			},
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
		url: '',
		method: '',
	},
	methods: {
		isRequired(value) {
			return value ? true : 'This field is required';
		},
		onSubmit() {
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
	}
};
</script>