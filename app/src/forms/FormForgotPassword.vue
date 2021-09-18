<template>
	<div>
		<VForm @submit='onSubmit' v-slot='{ errors }' class='vstack gap-2'>

			<label for='email' class='form-label'>The email you used to register</label>
			<div class='input-group' :class='{ "has-validation": errors.email }'>
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
				<button type='submit' :disabled='sending' class='btn btn-primary gradient'>{{ submitLabel }}</button>
				<ErrorMessage name='email' class='invalid-feedback shake' />
			</div>

		</VForm>

		<div class='mt-label'>
			Suddenly remember? <router-link :to='{ name: "login" }'>Log in</router-link>
		</div>
	</div>
</template>

<script>
	export default {
		name: 'FormForgotPassword',

		data() {
			return {
				sending: false,
				form: {
					email: '',
				},
			}
		},

		computed: {
			submitLabel() {
				return this.sending ? 'Requesting' : 'Request'
			},
		},

		methods: {
			async onSubmit() {
				this.sending = true

				const success = await this.$api.users.password.requestReset(this.form)
				if (success) this.$emit('success', success)

				this.sending = false
			},
		}
	}
</script>
