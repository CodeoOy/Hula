<template>
	<VForm @submit='onSubmit' v-slot='{ errors }' class='vstack gap-2'>

		<div>
			<label for='password' class='form-label'>Password</label>
			<VField
				v-model='form.password'
				type='password'
				id='password'
				name='password'
				label='Password'
				aria-label='Password'
				class='form-control'
				:class='{ "is-invalid": errors.password }'
				:rules="validatePassword"
			/>
			<small>Required: min 8 characters, uppercase letter, lowercase letter, number</small>
			<ErrorMessage name='password' class='invalid-feedback shake' />
		</div>

		<div>
			<label for='password_confirmation' class='form-label'>Repeat password</label>
			<VField
				rules='confirmed:@password'
				type='password'
				id='password_confirmation'
				name='password_confirmation'
				label='Password confirmation'
				aria-label='Password confirmation'
				class='form-control'
				:class='{ "is-invalid": errors.password_confirmation }'
			/>
			<ErrorMessage name='password_confirmation' class='invalid-feedback shake' />
		</div>

		<div class='mt-label'>
			<button type='submit' :disabled='sending || errors.password || errors.password_confirmation' class='btn btn-primary gradient float-end'>{{ submitLabel }}</button>
		</div>
	</VForm>
</template>

<script>
	export default {
		name: 'FormResetPassword',

		props: {
			id: {
				type: String,
				required: true,
			},
			email: {
				type: String,
				required: true,
			},
		},

		data() {
			return {
				sending: false,
				form: {
					...this.$props,
					password: '',
				}
			}
		},

		computed: {
			submitLabel() {
				return this.sending ? 'Changing' : 'Change'
			},
		},

		methods: {
			async onSubmit() {
				this.sending = true

				for (const prop in this.form) if (this.form[prop] == '') this.form[prop] = undefined
				const success = await this.$api.users.password.save(this.form)
				
				const message = success ? {
					type: 'success',
					title: 'Password changed',
				} : {
					type: 'error',
					title: 'Updating password failed',
				}

				this.$flashMessage.show({
					...message,
					time: 5000,
				})
				
				if (success) this.$emit('success', success)

				this.sending = false
			},
			validatePassword(value) {
				if (!value || value.length < 8) {
					return 'Minimum length is 8'
				}

				const lowercase = new RegExp("[a-z]");
				if (!lowercase.test(value)) {
					return 'Lowercase letter is required';
				}

				const uppercase = new RegExp("[A-Z]");
				if (!uppercase.test(value)) {
					return 'Uppercase letter is required';
				}

			 	const number = new RegExp("[0-9]");
				if (!number.test(value)) {
					return 'Number is required';
				}

				return true;
			}
		}
	}
</script>
