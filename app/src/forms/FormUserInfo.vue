<template>	
	<VForm v-on:submit='onSubmit'>

		<div class='mb-2 form-check' v-if='form.id'>
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

		<div class='mb-2 form-check' v-if='$store.state.loggeduser.isadmin'>
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

		<div class='mb-2 form-check' v-if='$store.state.loggeduser.isadmin'>
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

		<div class='mb-2'>
			<label for='firstname' class='form-label'>First name</label>
			<ErrorMessage name='firstname' class='error' />
			<VField
				v-model='form.firstname'
				rules='required'
				type='text'
				id='firstname'
				name='firstname'
				label='First name'
				aria-label='First name'
				class='form-control'
			/>
		</div>

		<div class='mb-2'>
			<label for='lastname' class='form-label'>Last name</label>
			<ErrorMessage name='lastname' class='error' />
			<VField
				v-model='form.lastname'
				rules='required'
				type='text'
				id='lastname'
				name='lastname'
				label='Last name'
				aria-label='Last name'
				class='form-control'
			/>
		</div>

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

		<button type='submit' class='btn btn-gradient'>Save</button>
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
				form: { ...this.$props },
			}
		},

		methods: {
			async onSubmit() {
				const user = await this.$api.users.save(this.form)
				if (user) this.$emit('success', user)
			},
		},
	}
</script>