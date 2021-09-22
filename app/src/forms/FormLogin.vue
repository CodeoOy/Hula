<template>
	<div>
		<VForm @submit='onSubmit' v-slot='{ errors }' class='vstack gap-2'>

			<div>
				<label for='email' class='form-label'>Email</label>
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

			<div>
				<label for='password' class='form-label me-3'>Password</label>
				<VField
					v-model='form.password'
					rules='required'
					type='password'
					id='password'
					name='password'
					label='Password'
					aria-label='Password'
					class='form-control'
					:class='{ "is-invalid": errors.required }'
				/>
				<ErrorMessage name='password' class='invalid-feedback shake' />
			</div>

			<div class='mt-label d-flex gap-3 align-items-center justify-content-between flex-wrap'>
				<button type='submit' :disabled='sending' class='btn btn-primary gradient align-self-start w-100 w-sm-auto order-sm-last'>{{ submitLabel }}</button>
				<div class='d-flex gap-3 mt-3 mt-sm-0'>
					<div><router-link :to='{ name: "forgot-password" }'>Forgot password?</router-link></div>
					<div class='vr' />
					<div>No account? <router-link :to='{ name: "register" }'>Sign&nbsp;up</router-link></div>
				</div>
			</div>
		</VForm>
	</div>
</template>

<script>
	export default {
		name: 'FormLogin',

		data() {
			return {
				sending: false,
				form: {
					email: '',
					password: '',
				},
			}
		},

		computed: {
			submitLabel() {
				return this.sending ? 'Logging in' : 'Log in'
			},
		},

		methods: {
			async onSubmit() {
				this.sending = true

				const success = await this.$store.dispatch('login', this.form)
				if (success) this.$emit('success', success)

				this.sending = false
			},
		},
	}
</script>
