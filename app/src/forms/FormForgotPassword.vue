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
		resetPassword() {
			fetch('/api/resetpassword', {
				method: 'POST',
				headers: {"Content-Type": "application/json"},
				credentials: 'include',
				body: JSON.stringify(this.formData)
			})
			.then(response => { 
				return (response.status >= 200 && response.status <= 299) ? response : this.$store.commit('errorHandling', response)
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