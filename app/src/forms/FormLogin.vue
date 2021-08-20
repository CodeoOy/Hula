<template>
	<div>
		<h2 class="h2">Log in</h2>
		<v-form v-on:submit="loginUser">
			<div class="mb-2">
				<label for="loginUser" class="form-label">Email</label>
				<error-message name="email" class="error"></error-message>
				<v-field
					v-model="email"
					:rules="isRequired"
					as="input"
					name="email"
					type="email"
					class="form-control"
					id="loginUser"
					aria-label="email" 
				></v-field>
			</div>
			<div class="mb-2">
				<label for="loginPassword" class="form-label">Password</label>
				<error-message name="password" class="error"></error-message>
				<v-field
					v-model="password"
					:rules="isRequired"
					as="input"
					name="password"
					type="password"
					class="form-control"
					id="loginPassword"
					aria-label="password" 
				></v-field>
			</div>
			<button type="submit" class="btn btn-gradient mb-1">Login</button>
		</v-form>
	</div>
</template>

<script>
	import { Field, Form, ErrorMessage } from 'vee-validate';
	export default {
		name: 'FormLogin',
		data() {
			return {
				user: {},
				email: '',
				password: ''
			}
		},
		components: {
			'VForm': Form,
			'VField': Field,
			ErrorMessage
		},
		methods: {
			isRequired(value) {
				return value ? true : 'This field is required';
			},

			async loginUser() {
				let data = {
					email: this.email,
					password: this.password,
				}

				const success = await this.$store.dispatch('login', data)

				if (success) {
					this.$router.push(this.$store.state.nextpage || { name: 'page-home' })
				}
			},
		},

		mounted() {
			this.$store.dispatch('setUser', null)
		}
	}
</script>