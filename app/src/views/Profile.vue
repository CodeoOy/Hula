<template>
	<div class="container mt-4">
		<div class="row gx-4">
			<div class="col-md-4">
				<div class="p-3 mb-4 rounded-2 content-box bg-dark text-light">
					<h1 class="h1">{{ user.firstname }} {{ user.lastname }}</h1>
					<p>{{ user.email }}</p>
					<a href="#" v-on:click.prevent="editUser(user)"><i class="bi-pencil-fill me-2"></i></a>
					<a href="#" v-on:click.prevent="confirmDelete('user', user)"><i class="bi-trash-fill me-2"></i></a>
					<hr />
					<VForm v-on:submit="saveFiles" class='clearfix'>
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
										<td><a href="#" v-on:click.prevent="confirmDelete('user.file', file)"><i class="bi-trash-fill me-2"></i></a></td>
									</tr>
								</tbody>
							</table>
							<label class="form-label">Upload files</label>
							<VField
								type="file"
								name="newFiles[]"
								multiple
								class="form-control" 
								v-model="newFiles"
							/>
						</div>
						<button type="submit" class="btn btn-gradient float-end">Upload files</button>
					</VForm>
				</div>
			</div>
			<div class="col-md-8">
				<div class="p-3 mb-4 rounded-2 content-box bg-dark text-light">
					<div class="d-sm-flex flex-row justify-content-between align-items-start">
						<h3 class="h3">Skills</h3>
						<button class="btn btn-gradient" v-on:click.prevent="editSkill()">Add skill</button>
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
										<a href="#" v-on:click.prevent="editSkill(skill)"><i class="bi-pencil-fill me-2"></i></a>
										<a href="#" v-on:click.prevent="confirmDelete('user.skill', skill)"><i class="bi-trash-fill me-2"></i></a>
									</td>
								</tr>
							</tbody>
						</table>
					</div>
					<div class="d-sm-flex flex-row justify-content-between align-items-start">
						<h3 class="h3">Reservations</h3>
						<button class="btn btn-gradient" v-on:click.prevent="editReservation()">Add reservation</button>
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
									<a href="#" v-on:click.prevent="editReservation(reservation)"><i class="bi-pencil-fill me-2"></i></a>
									<a href="#" v-on:click.prevent="confirmDelete('user.reservation', reservation)"><i class="bi-trash-fill me-2"></i></a>
								</td>
							</tr>
						</tbody>
					</table>
				</div>
				<div v-if='matches.length' class="p-3 mb-4 rounded-2 content-box bg-dark text-light">
					<h3 class="h3">Projects matching the skills</h3>
					<div class="table-responsive">
						<table class="table table-dark table-striped text-light">
							<thead>
								<tr>
									<th scope="col">Project</th>
									<th scope="col">Skills</th>
								</tr>
							</thead>
							<tbody>
								<tr v-for="match in matches" :key="match.id">
									<td><router-link :to='{ name: "page-project", params: { id: match.id } }'>{{ match.name }}</router-link></td>
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
	import FormUserInfo from '../forms/FormUserInfo.vue'
	import FormUserSkill from '../forms/FormUserSkill.vue'
	import FormUserReservation from '../forms/FormUserReservation.vue'

	export default {
		name: 'Profile',

		data() {
			return {
				user: {},
				reservations: [],
				files: [],
				newFiles: [],
			}
		},

		computed: {
			matches() {
				return this.$store.state.projects.filter(project => {
					return project.matches.some(match =>
						match.user_id == this.user.id
					)
				})			
			},
		},

		async mounted() {
			if (this.$route.params.id != this.$store.state.loggeduser.id) {
				if (!this.$store.state.loggeduser.isadmin) this.$router.push({ name: 'page-error' })
			}

			if (!this.$store.state.projects.length) this.$store.dispatch('getProjects')

			await Promise.all([
				this.getUser(),
				this.getReservations(),
				this.getUploads(),
			])
		},

		methods: {
			async getUser() {
				const promises = [ this.$api.users.get(this.$route.params.id) ]
				if (!this.$store.state.skillLevels.length) promises.push(this.$store.dispatch('getSkillLevels'))

				const [ user ] = await Promise.all(promises)

				this.user = user

				this.user.skills.forEach(skill => {
					skill.levelLabel = this.$store.state.skillLevels.find(({ id }) => id == skill.skillscopelevel_id).label
				})
			},

			async getReservations() {
				this.reservations = await this.$api.users.reservations.get(this.$route.params.id)
			},

			async getUploads() {
				const files = await this.$api.users.files.get(this.$route.params.id)
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

				if (success) this.getUploads()
			},

			async setCV(id) {
				const value = this.user.main_upload_id == id ? null : id

				const data = await this.$api.users.save({
					...this.user,
					main_upload_id: value,
				})

				if (data) this.user.main_upload_id = value
			},

			async editUser(props = {}) {
				const result = await this.$modal({
					title: 'Edit user info',
					component: FormUserInfo,
					props,
				})

				if (result) this.getUser()
			},

			async editSkill(props = {}) {
				props.user_id = this.user.id

				const result = await this.$modal({
					title: props.id ? `Edit skill: ${props.skill_label}` : 'Add skill',
					component: FormUserSkill,
					props,
				})

				if (result) this.getUser()
			},

			async editReservation(props = {}) {
				props.user_id = this.user.id

				const result = await this.$modal({
					title: props.id ? `Edit reservation: ${props.description}` : 'Add reservation',
					component: FormUserReservation,
					props,
				})

				if (result) this.getReservations()
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
							this.getUser()
							break
						
						case 'user.reservation':
							this.getReservations()
							break
						
						case 'user.file':
							this.getUploads()
							break
					}
				}
			}
		},
	}
</script>
