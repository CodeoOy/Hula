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
	</form>
</template>

<script>
	export default {
		name: 'Login',
		data() {
			return {
				message: "Click here",
				user: {},
			}
		},
		methods: {
			login: async function(e) { 
				e.preventDefault()    
				let email = e.target.elements.email.value
				let password = e.target.elements.password.value 
				let getUserData = () => {   
					let data = {    
						"email": email,    
						"password": password
					}
					fetch('api/auth', {method: 'POST', headers: {"Content-Type": "application/json"}, body: JSON.stringify(data)})
					.then((response) => response.json())
					.then((response) => {    
						console.log("Vue got Response");
						console.log("Response data: " + response);
						localStorage.setItem('user', JSON.stringify(response));
						this.$flashMessage.show({
							type: 'success',
							title: 'Successfully logged in',
							time: 1000
						});
						this.$router.push({path: '/'});
					})
					.catch((errors) => {
						//console.log("Vue got Error");
						console.log("Error data: " + errors);
						this.$flashMessage.show({
							type: 'error',
							title: 'Incorrect credentials',
							time: 1000
						});
						console.log("About to log out")
						this.$router.push({path: '/'});
					})    
				}    
				getUserData()    
			},
			switchForm: function() {

			}
		}
	}
</script>