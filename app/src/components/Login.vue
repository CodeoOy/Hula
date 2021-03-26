<template>
	<form v-on:submit="login">
		<div class="mb-3">
			<label for="exampleInputUser" class="form-label">Username</label>
			<input type="text" class="form-control" id="exampleInputUser" aria-describedby="emailHelp" name="username">
			<div id="emailHelp" class="form-text">We'll never share your email with anyone else.</div>
		</div>
		<div class="mb-3">
			<label for="exampleInputPassword" class="form-label">Password</label>
			<input type="password" class="form-control" id="exampleInputPassword" name="password">
		</div>
		<div class="mb-3 form-check">
			<input type="checkbox" class="form-check-input" id="exampleCheck">
			<label class="form-check-label" for="exampleCheck" name="checkbox">I like pina coladas and getting caught in the rain.</label>
		</div>
		<button type="submit" class="btn btn-primary">Login</button>
	</form>
</template>

<script>
	export default {
		name: 'Login',
		data() {
			return {
				message: "Click here",
				user: {}
			}
		},
		methods: {
			login: async function(e) { 
				e.preventDefault()    
				let username = e.target.elements.username.value
				let password = e.target.elements.password.value 
				let getUserData = () => {   
					let data = {    
						"username": username,    
						"password": password,
						"checkbox": checkbox
					}
					fetch('user/login', {method: 'POST', headers: {"Content-Type": "application/json"}, body: JSON.stringify(data)})
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
						//console.log("Error data: " + errors);
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
			}
		}
	}
</script>