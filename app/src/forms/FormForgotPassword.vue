<template>
	<VForm @submit='onSubmit' v-slot='{ errors }'>

		<div class='mb-2'>
			<label for='email' class='form-label'>The email you used to register</label>
			<VField
				v-model='form.email'
				rules='required|email'
				type='email'
				id='email'
				name='email'
				label='Email'
				aria-label='Email'
				class='form-control'
				:class='{ "is-invalid": errors.email }'
			/>
			<ErrorMessage name='email' class='invalid-feedback shake' />
		</div>

		<button type='submit' class='btn btn-primary gradient mb-1'>Submit</button>
	</VForm> 
</template>

<script>
	export default {
		name: 'FormForgotPassword',

		data () {
			return {
				form: {
					email: '',
				},
			}
		},

		methods: {
			async onSubmit() {
				const success = await this.$api.users.password.requestReset(this.form)
				if (success) this.$emit('success', success)
			},
		}
	}
</script>