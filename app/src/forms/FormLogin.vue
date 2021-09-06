<template>
	<div>
		<VForm v-on:submit='onSubmit'>

			<div class='mb-2'>
				<label for='email' class='form-label'>Email</label>
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

			<button type='submit' class='btn btn-gradient mb-1'>Login</button>
		</VForm>
	</div>
</template>

<script>
	export default {
		name: 'FormLogin',

		data() {
			return {
				form: {
					email: '',
					password: '',
				},
			}
		},

		methods: {
			async onSubmit() {
				const success = await this.$store.dispatch('login', this.form)
				if (success) this.$emit('success')
			},
		},
	}
</script>