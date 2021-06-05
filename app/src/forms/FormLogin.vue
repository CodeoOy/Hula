<template>
	<form v-on:submit="login">
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
			loginUser: function(email, password) {   
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
					if (response.ok) {
						fetch('/api/auth', {method: 'GET'})
						.then((response) => response.json())
						.then((response) => {
							this.$store.commit('setUser', response)
							this.$emit('loggedin')
							this.$flashMessage.show({
								type: 'success',
								title: 'Successfully logged in',
								time: 1000
							});
						})
					} else {
						fetch('/api/auth', {method: 'DELETE'})
						this.$flashMessage.show({
							type: 'error',
							title: 'Bad credentials. Cookies maybe deleted.',
							time: 1000
						});
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