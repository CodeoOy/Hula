<template>	
	<VForm @submit='onSubmit' v-slot='{ errors }' class='vstack gap-2'>

		<div class='form-check' v-if='form.id'>
			<label for='is_hidden' class='form-label'>Hidden</label>
			<VField
				v-model='form.is_hidden'
				:value='true'
				:unchecked-value='false'
				type='checkbox'
				id='is_hidden'
				name='is_hidden'
				class='form-check-input'
			/>
		</div>

		<div class='form-check' v-if='$store.state.loggeduser.isadmin'>
			<label for='isadmin' class='form-label'>Admin</label>
			<VField
				v-model='form.isadmin'
				:value='true'
				:unchecked-value='false'
				type='checkbox'
				id='isadmin'
				name='isadmin'
				class='form-check-input'
			/>
		</div>

		<div class='form-check' v-if='$store.state.loggeduser.isadmin'>
			<label for='is_employee' class='form-label'>Employee</label>
			<VField
				v-model='form.is_employee'
				:value='true'
				:unchecked-value='false'
				type='checkbox'
				id='is_employee'
				name='is_employee'
				class='form-check-input'
			/>
		</div>

		<div>
			<label for='firstname' class='form-label'>First name</label>
			<VField
				v-model='form.firstname'
				rules='required'
				type='text'
				id='firstname'
				name='firstname'
				label='First name'
				aria-label='First name'
				class='form-control'
				:class='{ "is-invalid": errors.firstname }'
			/>
			<ErrorMessage name='firstname' class='invalid-feedback shake' />
		</div>

		<div>
			<label for='lastname' class='form-label'>Last name</label>
			<VField
				v-model='form.lastname'
				rules='required'
				type='text'
				id='lastname'
				name='lastname'
				label='Last name'
				aria-label='Last name'
				class='form-control'
				:class='{ "is-invalid": errors.lastname }'
			/>
			<ErrorMessage name='lastname' class='invalid-feedback shake' />
		</div>

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

		<div class='mt-label'>
			<button type='submit' :disabled='sending' class='btn btn-primary gradient float-end'>{{ submitLabel }}</button>
		</div>
	</VForm>
</template>

<script>
	export default {
		name: 'FormUserInfo',

		props: {	
			id: {
				type: String,
				default: undefined,
			},
			is_hidden: {
				type: Boolean,
				default: false,
			},
			isadmin: {
				type: Boolean,
				default: false,
			},
			is_employee: {
				type: Boolean,
				default: false,
			},
			firstname: String,
			lastname: String,
			email: String,
		},

		data() {
			return {
				sending: false,
				form: { ...this.$props },
			}
		},

		computed: {
			submitLabel() {
				return this.sending ? 'Saving' : 'Save'
			},
		},

		methods: {
			async onSubmit() {
				this.sending = true

				const user = await this.$api.users.save(this.form)
				if (user) this.$emit('success', user)

				this.sending = false
			},
		},
	}
</script>
