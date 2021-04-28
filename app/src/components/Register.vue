<template>
	<form v-on:submit="register">
		<div class="mb-3">
			<label for="exampleInputUser" class="form-label">Email</label>
			<input type="text" class="form-control" id="exampleInputUser" aria-describedby="emailHelp" name="email">
			<div id="emailHelp" class="form-text">We'll never share your email with anyone else.</div>
		</div>
		<div class="mb-3">
			<label for="exampleInputPassword" class="form-label">Password</label>
			<input type="password" class="form-control" id="exampleInputPassword" name="password_plain">
		</div>
		<button type="submit" class="btn btn-gradient mb-1">Register</button>
	</form>
</template>

<script>
	export default {
		name: 'Register',
		data() {
			return {
				message: "Click here",
				user: {}
			}
		},
		methods: {
			register: async function(e) { 

				e.preventDefault()    
				let email = e.target.elements.email.value
				let password_plain = e.target.elements.password_plain.value 
				let getUserData = () => {   
					let data = {    
						"email": email,
						"password_plain": password_plain
					}
					fetch('api/invitation', {method: 'POST', headers: {"Content-Type": "application/json"}, body: JSON.stringify(data)})
					.then((response) => response.json())
					.then((response) => {    
						//console.log("Vue got Response");
						//console.log("Response data: " + response);
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
							title: 'No good',
							time: 1000
						});
						this.$router.push({path: '/'});
					})    
				}    
				getUserData()    
			}
		}
	}
</script>