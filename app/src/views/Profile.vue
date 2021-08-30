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
						v-on:click.prevent="confirmDelete('user', user)"
					><i class="bi-trash-fill me-2"></i></a>
					<hr />
					<v-form v-on:submit="saveFiles" class='clearfix'>
						<div class="mb-3">
							<table v-if='files.length' class="table table-dark table-striped text-light">
								<thead>
									<tr>
										<th scope="col">CV</th>
										<th scope="col" colspan='2'>File</th>
									</tr>
								</thead>
								<tbody>
									<tr v-for="file in files" :key="file.id">
										<td><input type='checkbox' :checked='user.main_upload_id == file.id' @click='setCV(file.id)'></td>
										<td><a href='#' @click.prevent>{{ file.filename }}</a></td>
										<td>
											<a 
												href="#"
												v-on:click.prevent="confirmDelete('user.file', file)"
											><i class="bi-trash-fill me-2"></i></a>
										</td>
									</tr>
								</tbody>
							</table>

							<label class="form-label">Upload files</label>
							<error-message name="files" class="error"></error-message>
							<v-field
								type="file"
								name="newFiles[]"
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
					<div class="d-sm-flex flex-row justify-content-between align-items-start">
						<h3 class="h3">Skills</h3>
						<button
							class="btn btn-gradient"
							v-on:click="formTitle = 'Add Skill', chosenSkill = {}, chosenForm = 'Skill', url = `/api/userskills/${user.id}`, method = 'POST'" 
							data-bs-toggle="modal" 
							data-bs-target="#hulaModalProfile"
						>Add skill</button>
					</div>
					<div class="table-responsive">
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
									<td>{{ skill.levelLabel }}</td>
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
											v-on:click.prevent="confirmDelete('user.skill', skill)"
										><i class="bi-trash-fill me-2"></i></a>
									</td>
								</tr>
							</tbody>
						</table>
					</div>
					<div class="d-sm-flex flex-row justify-content-between align-items-start">
						<h3 class="h3">Reservations</h3>
						<button
							class="btn btn-gradient"
							v-on:click="formTitle = 'Add Reservation', chosenReservation = {}, chosenForm = 'Reservation', url = `/api/userreservations/${user.id}`, method = 'POST'" 
							data-bs-toggle="modal" 
							data-bs-target="#hulaModalProfile"
						>Add reservation</button>
					</div>
					<table class="table table-dark table-striped text-light" v-if="reservations.length">
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
							<tr v-for="reservation in reservations" :key="reservation.id">
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
										v-on:click.prevent="confirmDelete('user.reservation', reservation)"
									><i class="bi-trash-fill me-2"></i></a>
								</td>
							</tr>
						</tbody>
					</table>
					<h3 class="h3">Matches</h3>
					<div class="table-responsive">
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
	</div>
</template>

<script>
	import VModal from '../components/VModal.vue'
	import { Modal } from 'bootstrap'
	import FormUserSkill from '../forms/FormUserSkill.vue'
	import FormUserReservation from '../forms/FormUserReservation.vue'
	import FormSkill from '../forms/FormSkill.vue'
	import FormUserBasicInfo from '../forms/FormUserBasicInfo.vue'
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
				skillLevels: [],
				url: '',
				method: '',
				reservations: [],
				files: [],
				newFiles: [],
			}
		},
		components: {
			FormUserBasicInfo,
			FormUserSkill,
			FormUserReservation,
			FormSkill,
			VModal,
			'VForm': Form,
			'VField': Field,
			ErrorMessage
		},
		methods: {
			hideModalUpdate() {
				this.checkProfile(this.$route.params.id)
				let modal = Modal.getInstance(document.querySelector('#hulaModalProfile'))
				modal.hide()
			},
			async checkProfile(id) {
				const [
					user,
					reservations,
				] = await Promise.all([
					this.$api.users.get(id),
					this.$api.users.reservations.get(id),
					this.getUserUploads(id),
				])
				this.user =  user
				this.reservations = reservations

				this.user.skills.forEach(skill => {
					skill.levelLabel = this.skillLevels.find(({ id }) => id == skill.skillscopelevel_id).label
				})
			},
			async getUserUploads(id) {
				const files = await this.$api.users.files.get(id)
				this.files = files
			},
			async saveFiles() {
				const success = await this.$api.users.files.save(this.newFiles)

				const message = success ? {
					type: 'success',
					title: 'Upload complete',
					text: `${this.newFiles.length} file${this.newFiles.length > 1 ? 's were' : ' was'} uploaded successfully`,
				} : {
					type: 'error',
					title: 'Upload failed',
				}

				this.$flashMessage.show({
					...message,
					time: 5000,
				})

				this.newFiles = []

				if (success) this.getUserUploads(this.$route.params.id)
			},
			async setCV(id) {
				const value = this.user.main_upload_id == id ? null : id

				const data = await this.$api.users.save({
					...this.user,
					main_upload_id: value,
				})

				if (data) this.user.main_upload_id = value
			},
			async confirmDelete(type, data) {
				const success = await this.$confirm.delete(type, data)
				
				if (success) {
					switch (type) {
						case 'user':
							if (data.id == this.$store.state.loggeduser.id) await this.$api.users.log.out()
							this.$router.push({ name: 'admin-users' })
							break

						case 'user.skill':
							this.user = await this.$api.users.get(this.$route.params.id)
							break
						
						case 'user.reservation':
							this.reservations = await this.$api.users.reservations.get(this.$route.params.id)
							break
						
						case 'user.file':
							this.getUserUploads(this.$route.params.id)
							break
					}
				}
			}
		},
		computed: {
			modalComponent() {
				const components = {
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
				return this.$store.state.projects.filter(project => {
					return project.matches.some(match =>
						match.user_id == this.user.id
					)
				})			
			},
		},
		async mounted() {
			if (this.$route.params.id != this.$store.state.loggeduser.id) {
				if (this.$store.state.loggeduser.isadmin !== true) {
					this.$router.push({name: 'page-error'})
				}
			}
			this.checkProfile(this.$route.params.id)
			this.skillLevels = await this.$api.skills.levels.get()
		}
	}
</script>
