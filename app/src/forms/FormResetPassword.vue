<template>
	<VForm @submit='onSubmit' v-slot='{ errors }'>

		<div class='mb-2'>
			<label for='password' class='form-label'>Password</label>
			<VField
				v-model='form.password'
				rules='required'
				type='password'
				id='password'
				name='password'
				label='Password'
				aria-label='Password'
				class='form-control'
				:class='{ "is-invalid": errors.password }'
			/>
			<ErrorMessage name='password' class='invalid-feedback shake' />
		</div>

		<div class='mb-2'>
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

		<button type="submit" class="btn btn-primary gradient mb-1">Submit</button>
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
				form: {
					...this.$props,
					password: '',
				}
			}
		},

		methods: {
			async onSubmit() {
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
			}
		}
	}
</script>