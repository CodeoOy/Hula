<template>
<div>
	<h2>Register</h2>
	<form v-on:submit="register">
		<div class="mb-3">
			<label for="inputEmail" class="form-label">Email</label>
			<input type="text" class="form-control" id="inputEmail" aria-describedby="emailHelp" name="email">
			<div id="emailHelp" class="form-text">We'll never share your email with anyone else.</div>
		</div>
		<div class="mb-3">
			<label for="inputFirstName" class="form-label">First name</label>
			<input type="text" class="form-control" id="inputFirstName" name="first_name">
		</div>
		<div class="mb-3">
			<label for="inputLastName" class="form-label">Last name</label>
			<input type="text" class="form-control" id="inputLastName" name="last_name">
		</div>
		<div class="mb-3">
			<label for="inputPassword" class="form-label">Password</label>
			<input type="password" class="form-control" id="inputPassword" name="password_plain">
		</div>
		<button type="submit" class="btn btn-gradient mb-1">Register</button>
	</form>
</div>
</template>

<script>
	export default {
		name: 'FormRegister',
		data() {
			return {
				user: {}
			}
		},
		methods: {
			register: async function(e) { 
				e.preventDefault()    
				let email = e.target.elements.email.value
				let first_name = e.target.elements.first_name.value 
				let last_name = e.target.elements.last_name.value 
				let password_plain = e.target.elements.password_plain.value 
				let getUserData = () => {   
					let data = {    
						"email": email,
						"password_plain": password_plain,
						"first_name": first_name,
						"last_name": last_name,
					}
					fetch('/api/invitations', {method: 'POST', headers: {"Content-Type": "application/json"}, body: JSON.stringify(data)})
					.then(response => { 
						return (response.status >= 200 && response.status <= 299) ? response.json() : this.$store.commit('errorHandling', response)
					})
					.then((response) => {    
						localStorage.setItem('user', JSON.stringify(response));
						this.$flashMessage.show({
							type: 'success',
							title: 'Invitation sent',
							time: 1000
						});
					})
					.catch((errors) => {
						console.log("Error data: " + errors);
						this.$flashMessage.show({
							type: 'error',
							title: 'Invitation not sent',
							time: 1000
						});
					})    
				}    
				getUserData()    
			}
		}
	}
</script>