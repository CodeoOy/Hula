<template>
	<VForm @submit='onSubmit' v-slot='{ errors }'>

		<div class='mb-2'>
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

		<div class='mb-2'>
			<label for='first_name' class='form-label'>First name</label>
			<VField
				v-model='form.first_name'
				rules='required'
				type='text'
				id='first_name'
				name='first_name'
				label='First name'
				aria-label='First name'
				class='form-control'
				:class='{ "is-invalid": errors.first_name }'
			/>
			<ErrorMessage name='first_name' class='invalid-feedback shake' />
		</div>

		<div class='mb-2'>
			<label for='last_name' class='form-label'>Last name</label>
			<VField
				v-model='form.last_name'
				rules='required'
				type='text'
				id='last_name'
				name='last_name'
				label='Last name'
				aria-label='Last name'
				class='form-control'
				:class='{ "is-invalid": errors.last_name }'
			/>
			<ErrorMessage name='last_name' class='invalid-feedback shake' />
		</div>

		<div class='mb-2'>
			<label for='password_plain' class='form-label'>Password</label>
			<VField
				v-model='form.password'
				rules='requiredNonAdmin'
				type='password'
				id='password_plain'
				name='password_plain'
				label='Password'
				aria-label='Password'
				class='form-control'
				:class='{ "is-invalid": errors.password_plain }'
			/>
			<ErrorMessage name='password_plain' class='invalid-feedback shake' />
		</div>

		<div class='mb-2'>
			<label for='password_confirmation' class='form-label'>Repeat password</label>
			<VField
				rules='confirmed:@password_plain'
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

		<button type='submit' class='btn btn-primary gradient mb-1'>Register</button>
	</VForm>
</template>

<script>
	export default {
		name: 'FormRegister',

		data() {
			return {
				form: {
					email: '',
					first_name: '',
					last_name: '',
					password_plain: '',
					password_pending: undefined,
				}
			}
		},

		methods: {
			async onSubmit() {
				this.form.password_pending = !this.form.password_plain

				const success = await this.$api.users.registration.invite(this.form)

				if (success) {
					this.$emit('success', success)

					this.$flashMessage.show({
						type: 'success',
						title: 'Invitation sent. Check your email',
						time: 5000,
					})
				}
			}
		}
	}
</script>