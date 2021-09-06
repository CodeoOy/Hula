<template>
	<VForm v-on:submit='onSubmit'>

		<div class='mb-2'>
			<label for='email' class='form-label'>The email you used to register</label>
			<ErrorMessage name='email' class='error' />
			<VField
				v-model='form.email'
				rules='required|email'
				type='email'
				id='email'
				name='email'
				label='Email'
				aria-label='Email'
				class='form-control'
			/>
		</div>

		<button type='submit' class='btn btn-gradient mb-1'>Submit</button>
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