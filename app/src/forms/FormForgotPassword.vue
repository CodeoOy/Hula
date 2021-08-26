<template>
	<v-form v-on:submit="resetPassword">
		<div class="mb-2">
			<label class="form-label">The email you used to register:</label>
			<error-message name="email" class="error"></error-message>
			<v-field
				v-model="formData.email"
				:rules="isRequired"
				as="input"
				type="text"
				name="email"
				class="form-control"
				aria-label="email"
			></v-field>
		</div>
		<button type="submit" class="btn btn-gradient mb-1">Submit</button>
	</v-form> 
</template>

<script>
import { Field, Form, ErrorMessage } from 'vee-validate';
export default {
	name: 'FormForgotPassword',
	data () {
        return {
            formData: {
				email: ''
			},
        }
    },
	components: {
		'VForm': Form,
		'VField': Field,
		ErrorMessage
	},
	methods: {
		isRequired(value) {
			return value ? true : 'This field is required'
		},

		async resetPassword() {
			const success = await this.$api.users.password.requestReset(this.formData)
			if (success) this.$emit('formSent')
		},
	}
};
</script>