<template>
	<v-form v-on:submit="inviteUser">
		<div class="mb-2">
			<label class="form-label">Password</label>
			<error-message name="password" class="error"></error-message>
			<v-field
				v-model="formData.password"
				:rules="isRequired"
				as="input"
				type="text"
				name="password"
				class="form-control"
				aria-label="password"
			></v-field>
		</div>
		<div class="mb-2">
			<label class="form-label">Password again</label>
			<error-message name="passwordAgain" class="error"></error-message>
			<v-field
				v-model="formData.passwordAgain"
				:rules="isRequired"
				as="input"
				type="text"
				name="passwordAgain"
				class="form-control"
				aria-label="password"
			></v-field>
		</div>
		<button type="submit" class="btn btn-gradient mb-1">Submit</button>
	</v-form>
</template>

<script>
import { Field, Form, ErrorMessage } from 'vee-validate';
export default {
	name: 'FormResetPassword',
	data() {
		return {
			user: {},
			formData: {
				password: '',
				passwordAgain: '',
			}
		}
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
	}
}
</script>