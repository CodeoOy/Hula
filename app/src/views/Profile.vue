<template>
	<div class="container mt-4">
		<div class="modal fade" v-bind:class="{ 'show db': editing_skills, '': !editing_skills }">
			<div class="modal-dialog">
				<div class="modal-content p-3 rounded-2 content-box bg-dark text-light">
					<div>
						<a href="#" v-on:click="editing_skills = false" class="btn btn-gradient">Done</a>
					</div>
				</div>
			</div>
		</div>
		<div class="row gx-4">
			<div class="col-md-4">
				<div class="p-3 mb-4 rounded-2 content-box bg-dark text-light">
					<h1>{{ user.firstname }} {{ user.lastname }}</h1>
					<p>{{ user.email }}</p>
					<p>{{ user.telephone }}</p>
					<p>{{ user.id }}</p>
					<img src="" alt="">
				</div>
			</div>
			<div class="col-md-8">
				<div class="p-3 mb-4 rounded-2 content-box bg-dark text-light">
					<h2>Professional profile</h2>
					<h3>Basic info</h3>
					<a href="#" v-on:click="editing_info = true">Edit your info</a>
					<p>Available: {{ user.available }}</p>
					<transition name="fadeHeight">
						<UserForm :user='user' v-if="editing_info == true" v-on:formsent="editing_info = false, updateUser()"/>
					</transition>
					<h3>Skills</h3>
					<table class="table table-dark table-striped text-light">
						<thead>
							<tr>
								<th scope="col">Skill</th>
								<th scope="col">Level</th>
								<th scope="col">Actions</th>
							</tr>
						</thead>
						<tbody>
							<tr v-for="skill in user.skills" :key="skill.id">
								<td>{{ skill.skill_label }}</td>
								<td>{{ skill.years }}</td>
								<td>Edit - Delete</td>
							</tr>
						</tbody>
					</table>
					<a href="#" v-on:click="addExistingSkill, editing_skills = true">Add test skill</a>
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
				user: {},
				editing_info: false,
				editing_skills: false,
				testskill: {
					id: "e9becc32-0238-4561-b341-106de1c26666",
					user_id: "d757caa2-4128-4f5b-9638-bd433dc49444",
					skill_id: "e9becc32-0238-4561-b341-106de1c26060",
					skillscopelevel_id: "e9becc32-0238-4561-b341-106de1c26048",
					years: 10.0,
					updated_by: "psi"
				}
			}
		},
		components: {
			'UserForm': UserForm
		},
		methods: {
			addExistingSkill: function() {
				fetch(`http://localhost:8086/api/userskill/${this.user.id}`, {
					method: 'PUT',
					headers: {"Content-Type": "application/json"},
					credentials: 'include',
					body: JSON.stringify(this.testskill)
				})
			},
			updateUser: function() {   
				fetch(`http://localhost:8086/api/user/${this.user.id}`, {
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
			}
		},
		mounted() {
			this.user = this.$store.state.loggeduser
		}
	}
</script>