<template>
	<div class="container mt-4">
		<VModal :modalTitle="formTitle" :modalID="'Profile'" v-on:modal-hidden="chosenForm = '', chosenSkill = {}">
			<component 
				:is='modalComponent' 
				:url="url"
				:method="method"
				:chosenUser="user"
				:chosenSkill="chosenSkill"
				:chosenReservation="chosenReservation"
				:userSkills="userSkills"
				:userID="user.id"
				v-on:form-sent="hideModalUpdate"
			/>
		</VModal>
		<div class="row gx-4">
			<div class="col-md-4">
				<div class="p-3 mb-4 rounded-2 content-box bg-dark text-light">
					<h1 class="h1">{{ user.firstname }} {{ user.lastname }}</h1>
					<p>{{ user.email }}</p>
					<a 
						href="#"
						data-bs-toggle="modal" 
						data-bs-target="#hulaModalProfile" 
						v-on:click="formTitle = 'Edit user info', chosenForm = 'User', chosenUser = user, url=`/api/users/${user.id}`, method='PUT'"
					><i class="bi-pencil-fill me-2"></i></a>
					<a 
						href="#"
						data-bs-toggle="modal" 
						data-bs-target="#hulaModalProfile" 
						v-on:click="formTitle = 'Delete my profile', chosenForm = 'Delete', chosenUser = user, url=`/api/users/${user.id}`, method='DELETE'"
					><i class="bi-trash-fill me-2"></i></a>
					<hr />
					<v-form v-on:submit="uploadFile" class='clearfix'>
						<div class="mb-3">
							<table v-if='user.uploads && user.uploads.length' class="table table-dark table-striped text-light">
								<thead>
									<tr>
										<th scope="col">CV</th>
										<th scope="col" colspan='2'>File</th>
									</tr>
								</thead>
								<tbody>
									<tr v-for="file in user.uploads" :key="file.id">
										<td><input type='checkbox' :checked='user.main_upload_id == file.id' @click='setCV(file.id)'></td>
										<td><a href='#' @click.prevent>{{ file.filename }}</a></td>
										<td>
											<a 
												href="#"
												data-bs-toggle="modal" 
												data-bs-target="#hulaModalProfile" 
												v-on:click="formTitle = `Delete file ${file.filename}`, chosenForm = 'Delete', url=`/api/useruploads/${file.id}`, method='DELETE'"
											><i class="bi-trash-fill me-2"></i></a>
										</td>
									</tr>
								</tbody>
							</table>

							<label class="form-label">Upload files</label>
							<error-message name="files" class="error"></error-message>
							<v-field
								type="file"
								name="files[]"
								multiple
								class="form-control" 
								v-model="newFiles"
							></v-field>
						</div>
						<button type="submit" class="btn btn-gradient float-end">Upload files</button>
					</v-form>
				</div>
			</div>
			<div class="col-md-8">
				<div class="p-3 mb-4 rounded-2 content-box bg-dark text-light">
					<div class="d-flex flex-row justify-content-between align-items-start">
						<h3 class="h3">Skills</h3>
						<button
							class="btn btn-gradient"
							v-on:click="formTitle = 'Add Skill', chosenSkill = {}, chosenForm = 'Skill', url = `/api/userskills/${user.id}`, method = 'POST'" 
							data-bs-toggle="modal" 
							data-bs-target="#hulaModalProfile"
						>Add skill</button>
					</div>
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
										v-on:click="formTitle = `Delete ${skill.skill_label}?`, chosenSkill = skill, chosenForm = 'Delete', url = `/api/userskills/${skill.id}`, method = 'DELETE'"
									><i class="bi-trash-fill me-2"></i></a>
								</td>
							</tr>
						</tbody>
					</table>
					<div class="d-flex flex-row justify-content-between align-items-start">
						<h3 class="h3">Reservations</h3>
						<button
							class="btn btn-gradient"
							v-on:click="formTitle = 'Add Reservation', chosenReservation = {}, chosenForm = 'Reservation', url = `/api/userreservations/${user.id}`, method = 'POST'" 
							data-bs-toggle="modal" 
							data-bs-target="#hulaModalProfile"
						>Add reservation</button>
					</div>
					<table class="table table-dark table-striped text-light" v-if="'reservations' in user">
						<thead>
							<tr>
								<th scope="col">Description</th>
								<th scope="col">From</th>
								<th scope="col">To</th>
								<th scope="col">Percentage</th>
								<th scope="col">Actions</th>
							</tr>
						</thead>
						<tbody>
							<tr v-for="reservation in user.reservations" :key="reservation.id">
								<td>{{ reservation.description }}</td>
								<td>{{ reservation.begin_time }}</td>
								<td>{{ reservation.end_time }}</td>
								<td>{{ reservation.percentage }}</td>
								<td>
									<a 
										href="#"
										data-bs-toggle="modal"
										data-bs-target="#hulaModalProfile"
										v-on:click="formTitle = reservation.id, chosenForm = 'Reservation', chosenReservation = reservation, url=`/api/userreservations/${reservation.id}`, method='PUT'"
									><i class="bi-pencil-fill me-2"></i></a>
									<a
										href="#"
										data-bs-toggle="modal"
										data-bs-target="#hulaModalProfile" 
										v-on:click="formTitle = `Delete reservation?`, chosenReservation = reservation, chosenForm = 'Delete', url = `/api/userreservations/${reservation.id}`, method = 'DELETE'"
									><i class="bi-trash-fill me-2"></i></a>
								</td>
							</tr>
						</tbody>
					</table>
					<h3 class="h3">Matches</h3>
					<table class="table table-dark table-striped text-light">
						<thead>
							<tr>
								<th scope="col">Project</th>
								<th scope="col">Skills</th>
							</tr>
						</thead>
						<tbody>
							<tr v-for="match in getUserMatches" :key="match.id">
								<td>{{ match.name }}</td>
								<td><span class="badge badge-skill me-2" v-for="skill in match.skills" :key="skill.skill_id">{{ skill.skill_label }}</span></td>
							</tr>
						</tbody>
					</table>
				</div>
			</div>
		</div>
	</div>
</template>

<script>
	import VModal from '../components/VModal.vue'
	import { Modal } from 'bootstrap'
	import FormUserSkill from '../forms/FormUserSkill.vue'
	import FormUserReservation from '../forms/FormUserReservation.vue'
	import FormSkill from '../forms/FormSkill.vue'
	import FormUserBasicInfo from '../forms/FormUserBasicInfo.vue'
	import FormConfirmAction from '../forms/FormConfirmAction.vue'
	import { Field, Form, ErrorMessage } from 'vee-validate';
	export default {
		name: 'Profile',
		data() {
			return {
				formTitle: '',
				chosenForm: '',
				chosenUser: {},
				chosenSkill: {},
				chosenReservation: {},
				editingInfo: false,
				user: {},
				url: '',
				method: '',
				files: [
					{ id: '00000000-0000-0000-0000-000000000001', name: 'Fake file #01.jpeg' },
					{ id: '00000000-0000-0000-0000-000000000002', name: 'Fake file #02.jpeg' },
					{ id: '00000000-0000-0000-0000-000000000003', name: 'Breaking the pattern.gif' },
					{ id: '00000000-0000-0000-0000-000000000004', name: 'Fake file #04.jpeg' },
				],
				newFiles: [],
			}
		},
		components: {
			FormUserBasicInfo,
			FormUserSkill,
			FormUserReservation,
			FormSkill,
			FormConfirmAction,
			VModal,
			'VForm': Form,
			'VField': Field,
			ErrorMessage
		},
		methods: {
			hideModalUpdate() {
				this.checkProfile(this.$route.params.id)
				this.getUserReservations(this.$route.params.id)
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
					this.getUserReservations(this.$route.params.id)
					this.getUserUploads(id)
				}) 
			},
			getUserReservations(id) {
				fetch(`/api/userreservations/${id}`, {method: 'GET'})
				.then(response => {
					if (response.status == 204) {
						return []
					}
					return (response.status >= 200 && response.status <= 299) ? response.json() : this.$store.commit('errorHandling', response)
				})
				.then(response => {
					this.user.reservations = response;
				})
			},
			getUserUploads(id) {
				fetch(`/api/useruploads/${id}`, {method: 'GET'})
					.then(response => {
						if (response.status == 204) return []
						if (response.status >= 200 && response.status <= 299) return response.json()
						throw Error(response)
					})
					.then(response => {
						this.user.uploads = Array.isArray(response) ? response : [response]
					})
					.catch((errors) => {
						this.$store.commit('errorHandling', errors)
						this.user.uploads = []
					})
			},
			uploadFile() {
				var data = new FormData()
				if (this.newFiles.length) this.newFiles.forEach(file => data.append('files[]', file))

				fetch(`/api/upload`, {
					method: 'POST',
					//headers: {"Content-Type": "application/json"},
					credentials: 'include',
					body: data,
				})
				.then(response => {
					if (response.status >= 200 && response.status <= 299) {
						this.getUserUploads(this.$route.params.id)
					} else {
						this.$store.commit('errorHandling', response)
					}
				})
				.catch((errors) => {
					this.$store.commit('errorHandling', errors)
				})

				this.newFiles = []
			},
			setCV(id) {
				const value = this.user.main_upload_id == id ? null : id
				fetch(`/api/users/${this.user.id}`, {
					method: 'PUT',
					headers: {"Content-Type": "application/json"},
					credentials: 'include',
					body: JSON.stringify({ ...this.user, main_upload_id: value })
				})
				.then(response => {
					if (response.status >= 200 && response.status <= 299) {
						this.user.main_upload_id = value
					} else {
						this.$store.commit('errorHandling', response)
					}
				})
				.catch((errors) => {
					this.$store.commit('errorHandling', errors)
				})
			}
		},
		computed: {
			modalComponent() {
				const components = {
					Delete: FormConfirmAction,
					User: FormUserBasicInfo,
					Skill: FormUserSkill,
					Reservation: FormUserReservation,
				}
				return components[this.chosenForm]
			},
			userSkills() {
				return this.user.skills
			},
			getUserMatches() {
				console.log(this.user.id)
				return this.$store.state.projects.filter(project => {
					return project.matches.some(match =>
						match.user_id == this.user.id
					)
				})			
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
			this.checkProfile(this.$route.params.id)
		}
	}
</script>