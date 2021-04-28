<template>
	<div class="container mt-4">
		<div class="row gx-4">
			<div class="col-md-4">
				<div class="p-3 mb-4 rounded-2 content-box bg-dark text-light">
					<h1>{{ user.firstname }} {{ user.lastname }}</h1>
					<p>{{ user.email }}</p>
					<p>{{ user.telephone }}</p>
					<img src="" alt="">
				</div>
			</div>
			<div class="col-md-8">
				<div class="p-3 mb-4 rounded-2 content-box bg-dark text-light">
					<h2>Professional profile</h2>
					<a href="#" v-on:click="editing = true">Edit your info</a>
					<p>Available: {{ user.available }}</p>
					<transition name="fadeHeight">
						<UserForm :user='user' v-if="editing == true" v-on:formsent="editing = false, updateUser()"/>
					</transition>
				</div>
			</div>
		</div>
	</div>
</template>

<script>
	import UserForm from '../components/UserForm.vue'
	export default {
		name: 'Profile',
		data() {
			return {
				userid: {},
				user: {},
				editing: false
			}
		},
		components: {
			'UserForm': UserForm
		},
		methods: {
			getUserId: function () {
				this.userid = JSON.parse(localStorage.getItem('user'))
			},
			getUserData: function() { 
				fetch(`http://localhost:8086/api/user/${this.userid}`, {
					method: 'GET',
					headers: {"Content-Type": "application/json"}
				})
				.then((response) => response.json())
				.then(response => { 
					//console.log(response);
					this.user = response;
				})    
			},
			updateUser: function() {   
				fetch(`http://localhost:8086/api/user/${this.userid}`, {
					method: 'PUT',
					headers: {"Content-Type": "application/json"},
					credentials: 'include',
					body: JSON.stringify(this.user)
				})
				.then((response) => {
					if (response.ok) {
						fetch('http://localhost:8086/api/auth', {method: 'GET'})
						.then((response) => response.json())
						.then((response) => {
							this.message = response;
							localStorage.setItem('user', JSON.stringify(response));
							//console.log(localStorage.getItem('user'))
							this.$emit('loggedin')
							this.$flashMessage.show({
								type: 'success',
								title: 'User info saved',
								time: 1000
							});
						})
					} else {
						fetch('http://localhost:8086/api/auth', {method: 'DELETE'})
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
				this.updateUser(email, password);
			}
		},
		mounted() {
			console.log(this.$store.state.loggeduser)
			this.user = this.$store.state.loggeduser
			//this.getUserId()
			//this.getUserData(this.userid)
		}
	}
</script>