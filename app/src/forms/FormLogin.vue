<template>
	<div>
		<h2>Log in</h2>
		<form v-on:submit="login">
			{{ this.$store.state.loggeduser }}
			<div class="mb-3">
				<label for="loginUser" class="form-label">Email</label>
				<input type="text" class="form-control" id="loginUser" aria-describedby="emailHelp" name="email">
			</div>
			<div class="mb-3">
				<label for="loginPassword" class="form-label">Password</label>
				<input type="password" class="form-control" id="loginPassword" name="password">
			</div>
			<button type="submit" class="btn btn-gradient mb-1">Login</button>
		</form>
	</div>
</template>

<script>
	export default {
		name: 'FormLogin',
		data() {
			return {
				user: {},
			}
		},
		methods: {
			loginUser(email, password) {   
				let data = {    
					"email": email,    
					"password": password
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
						.then((data) => {
							this.$emit('hideModal')
							console.log(this.$store.state.loggeduser)
							this.$router.push({ name: 'page-home' })
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
			login: async function(e) { 
				e.preventDefault()    
				let email = e.target.elements.email.value
				let password = e.target.elements.password.value 
				this.loginUser(email, password);
			}
		}
	}
</script>