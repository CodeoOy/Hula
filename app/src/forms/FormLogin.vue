<template>
	<div>
		<h2>Log in</h2>
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
			loginUser() {
				console.log("Logging in")   
				let data = {    
					"email": this.email,    
					"password": this.password
				}
				fetch('/api/auth', {
					method: 'POST',
					headers: {"Content-Type": "application/json"},
					credentials: 'include',
					body: JSON.stringify(data)
				})
				.then((response) => {
					if(response.ok) {
						fetch('/api/auth', {method: 'GET'})
						.then((response) => response.json())
						.catch((errors) => {
							console.log("Auth (GET) failed, error data:");
							console.log(errors)
						}) 
						.then(async response => {
							console.log(response)
							await this.$store.dispatch('setUser', response)
						})
						.then(() => {
							this.$emit('hideModal')
							console.log(this.$store.state.loggeduser)
							if (this.$store.state.nextpage.length) {
								this.$router.push({ name: this.$store.state.nextpage })
							} else {
								this.$router.push({ name: 'page-home' })
							}
						})
					} else {
						this.$flashMessage.show({
							type: 'error',
							title: 'Unauthorized',
							time: 1000
						});
						throw new Error('Auth (POST) failed');
					}
				})
			},
			/*
			login: async function(e) { 
				e.preventDefault()    
				let email = e.target.elements.email.value
				let password = e.target.elements.password.value 
				this.loginUser(email, password);
			}*/
		}
	}
</script>