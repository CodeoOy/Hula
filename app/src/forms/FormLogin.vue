<template>
	<div>
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
					:class='{ "is-invalid": errors.required }'
				/>
				<ErrorMessage name='password' class='invalid-feedback shake' />
			</div>

			<button type='submit' class='btn btn-primary gradient mb-1'>Login</button>
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