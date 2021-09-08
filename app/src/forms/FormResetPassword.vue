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

		data() {
			return {
				form: {
					password: '',
				}
			}
		},

		methods: {
			onSubmit() {
				this.$emit('success', this.form.password)
			}
		}
	}
</script>