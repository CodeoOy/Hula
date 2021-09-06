<template>
	<VForm v-on:submit="onSubmit">

		<div class='mb-2'>
			<label for='password' class='form-label'>Password</label>
			<ErrorMessage name='password' class='error' />
			<VField
				v-model='form.password'
				rules='required'
				type='password'
				id='password'
				name='password'
				label='Password'
				aria-label='Password'
				class='form-control'
			/>
		</div>

		<div class='mb-2'>
			<label for='password_confirmation' class='form-label'>Repeat password</label>
			<ErrorMessage name='password_confirmation' class='error' />
			<VField
				rules='confirmed:@password'
				type='password'
				id='password_confirmation'
				name='password_confirmation'
				label='Password confirmation'
				aria-label='Password confirmation'
				class='form-control'
			/>
		</div>

		<button type="submit" class="btn btn-gradient mb-1">Submit</button>
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