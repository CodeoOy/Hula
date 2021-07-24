<template>
	<div class="container mt-4">
		<VModal :modalTitle="formTitle" :modalID="'Profile'" v-on:updated-modal="chosenForm = '', chosenSkill = {}">
			<component 
				:is='modalComponent' 
				:url="url"
				:method="method"
				:chosenSkill="chosenSkill"
				:userID="user.id"
				v-on:form-sent="hideModalUpdate"
			/>
		</VModal>
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
					<a href="#" v-on:click="editingInfo = true">Edit your info</a>
					<p>Hidden: {{ user.is_hidden }}</p>
					<transition name="fadeHeight">
						<FormUserBasicInfo :user='user' v-if="editingInfo == true" v-on:formsent="editingInfo = false, updateUser()"/>
					</transition>
					<h3>Skills</h3>
					<table class="table table-dark table-striped text-light">
						<thead>
							<tr>
								<th scope="col">Skill</th>
								<th scope="col">Level</th>
								<th scope="col">Years</th>
								<th scope="col">Actions</th>
							</tr>
						</thead>
						<tbody>
							<tr v-for="skill in user.skills" :key="skill.id">
								<td>{{ skill.skill_label }}</td>
								<td>{{ skill.skillscopelevel_id }}</td>
								<td>{{ skill.years }}</td>
								<td>
									<a 
										href="#"
										data-bs-toggle="modal"
										data-bs-target="#hulaModalProfile"
										v-on:click="formTitle = skill.skill_label, chosenForm = 'Skill', chosenSkill = skill, url=`/api/userskills/${skill.id}`, method='PUT'"
									><i class="bi-pencil-fill me-2"></i></a>
									<a
										href="#"
										data-bs-toggle="modal"
										data-bs-target="#hulaModalProfile" 
										v-on:click="formTitle = `Delete ${skill.skill_label}?`, chosenForm = 'Delete', url = `/api/userskills/${skill.id}`, method = 'DELETE'"
									><i class="bi-trash-fill me-2"></i></a>
								</td>
							</tr>
						</tbody>
					</table>
					<p><a href="#" 
						v-on:click="formTitle = 'Add Skill', chosenSkill = {}, chosenForm = 'Skill', url = `/api/userskills/${user.id}`, method = 'POST'" 
						data-bs-toggle="modal" 
						data-bs-target="#hulaModalProfile"
					>Add skill</a></p>
				</div>
			</div>
		</div>
	</div>
</template>

<script>
	import VModal from '../components/VModal.vue'
	import { Modal } from 'bootstrap'
	import FormUserSkill from '../forms/FormUserSkill.vue'
	import FormSkill from '../forms/FormSkill.vue'
	import FormUserBasicInfo from '../forms/FormUserBasicInfo.vue'
	import FormConfirmAction from '../forms/FormConfirmAction.vue'
	export default {
		name: 'Profile',
		data() {
			return {
				formTitle: '',
				chosenForm: '',
				chosenSkill: {},
				editingInfo: false,
				user: this.$store.state.loggeduser,
				url: '',
				method: '',
			}
		},
		components: {
			FormUserBasicInfo,
			FormUserSkill,
			FormSkill,
			FormConfirmAction,
			VModal,
		},
		methods: {
			updateUser() { // This is not working. Make the user update happen in some other way.
				fetch(`/api/users/${this.user.id}`, {
					method: 'PUT',
					headers: {"Content-Type": "application/json"},
					credentials: 'include',
					body: JSON.stringify(this.user)
				})
			},
			hideModalUpdate() {
				this.checkProfile(this.$route.params.id)
				let modal = Modal.getInstance(document.querySelector('#hulaModalProfile'))
				modal.hide()
			},
			deleteSkill(id) {
				fetch(`/api/userskills/${id}`, {
					method: 'DELETE',
					headers: {"Content-Type": "application/json"},
					credentials: 'include',
					body: JSON.stringify(this.user)
				})
			},
			checkProfile(id) {
				fetch(`/api/users/${id}`, {method: 'GET'})
				.then(response => { 
					return (response.status >= 200 && response.status <= 299) ? response.json() : this.$store.commit('errorHandling', response)
				})
				.then(response => { 
					this.user = response;
				}) 
			}
		},
		computed: {
			modalComponent() {
				const components = {
					Delete: FormConfirmAction,
					Skill: FormUserSkill,
				}
				return components[this.chosenForm]
			},
		},
		mounted() {
			if (this.$route.params.id != this.$store.state.loggeduser.id) {
				if (this.$store.state.loggeduser.isadmin === true) {
					this.checkProfile(this.$route.params.id)
				} else {
					this.$router.push({name: 'page-error'})
				}
			}
		}
	}
</script>