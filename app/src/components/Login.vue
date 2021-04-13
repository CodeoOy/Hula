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
		<!--<div class="mb-3 form-check">
			<input type="checkbox" class="form-check-input" id="exampleCheck">
			<label class="form-check-label" for="exampleCheck" name="checkbox">I like pina coladas and getting caught in the rain.</label>
		</div>-->
		<button type="submit" class="btn btn-primary text-light">Login</button>
		<p><a href="#" v-on:click="logoutUser">Log out test link</a></p>
		<p>{{ message }}</p>
	</form>
</template>

<script>
	export default {
		name: 'Login',
		data() {
			return {
				message: "Not logged in yet",
				user: {},
			}
		},
		methods: {
			loginUser: function(email, password) {   
				let data = {    
					"email": email,    
					"password": password
				}
				fetch('api/auth', {method: 'POST', headers: {"Content-Type": "application/json"}, credentials: 'include', body: JSON.stringify(data)})
				//.then((response) => response.json())
				//.then(response => {return response.json();})
				.then((response) => {    
					if (response.ok) {
						console.log("Vue got Response");
						console.log(response);
						this.message = response;
						//localStorage.setItem('user', JSON.stringify(response));
						this.$flashMessage.show({
							type: 'success',
							title: 'Successfully logged in',
							time: 1000
						});
						//this.$router.push({path: '/'});
						return true;
					} else {
						//console.log("Vue got Error");
						console.log("Error data: " + response);
						this.$flashMessage.show({
							type: 'error',
							title: 'Incorrect credentials',
							time: 1000
						});
						console.log("About to log out")
						//this.$router.push({path: '/'});
						return false;
					}
				})
			},
			getUser: function(email, password) {   
				let data = {    
					"email": email,    
					"password": password
				}
				fetch('api/auth', {method: 'POST', headers: {"Content-Type": "application/json"}, body: JSON.stringify(data)})
				.then((response) => {    
					if (response.ok) {
						console.log("Got user")
					} else {
						console.log("No user")
					}
				})
			},
			login: async function(e) { 
				e.preventDefault()    
				let email = e.target.elements.email.value
				let password = e.target.elements.password.value 
				this.loginUser(email, password);
				/*
				let getUser = () => {
					console.log("Henlo user.")
				}
				if (loginUser(email, password) == true) {
					
				} else {
					console.log("Nope.")
				}
				*/
			},
			logoutUser: function() {
				console.log("Trying to log out now.")
				fetch('api/auth', {method: 'DELETE'})
			}
		}
	}
</script>