<template>
	<div class="container">
		<button class='btn btn-primary mb-3 gradient float-start' v-on:click='home()'>Home</button>
		<div class="row gx-4">
			<div class="col-md-4">
				<div class="card shadow" :class='$colorScheme.card'>
					<div class='context'>
						<div class='card-header d-flex align-items-center'>
							<h1 class="h3 mb-0 flex-grow-1">
								{{ user.firstname }} {{ user.lastname }}
							</h1>
							<i v-if='user.is_hidden' class="bi-eye-slash-fill ms-3 fs-3 lh-1" title='Hidden user'></i>
						</div>
						<div class='card-body'>
							<div>{{ user.email }}</div>
							<div class='context-actions hstack gap-1 justify-content-end'>
								<button class='btn btn-unstyled px-1 rounded' v-on:click="editUser(user)"><i class="bi-pencil-fill" title='Edit profile'></i></button>
								<button class='btn btn-unstyled px-1 rounded' v-on:click="confirmDelete('user', user)"><i class="bi-trash-fill" title='Delete profile'></i></button>
							</div>
						</div>
					</div>
					<div class='card-body border-top border-inherit'>
						<VForm @submit="saveFiles" class='clearfix'>
							<div class="mb-3">
								<table v-if='files.length' class="table table-striped" :class='$colorScheme.table'>
									<thead>
										<tr>
											<th scope="col">CV</th>
											<th scope="col" colspan='2'>File</th>
										</tr>
									</thead>
									<tbody>
										<tr v-for="file in files" :key="file.id" class='context'>
											<td><input type='checkbox' :checked='user.main_upload_id == file.id' @click='setCV(file.id)'></td>
											<td><a :href='downloadUrl(file.id)' target='_blank' rel='noopener noreferrer'>{{ file.filename }}</a></td>
											<td class='text-end'>
												<div class='context-actions'>
													<button class='btn btn-unstyled px-1 rounded' v-on:click.prevent="confirmDelete('user.file', file)"><i class="bi-trash-fill" title='Delete file'></i></button>
												</div>
											</td>
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
							<button type='submit' :disabled='sending || !newFiles.length' class='btn btn-primary gradient float-end'>{{ submitLabel }}</button>
						</VForm>
					</div>
				</div>
			</div>
			<div class="mt-4 mt-md-0 col-md-8">
				<div class="card shadow" :class='$colorScheme.card'>
					<div class='card-header'>
						<div class="d-flex flex-wrap justify-content-between align-items-center">
							<h3 class="h3 mb-0">Skills</h3>
							<button class="btn btn-primary gradient" v-on:click="editSkill()">Add skill</button>
						</div>
					</div>
					<div class='card-body'>
						<div v-if='user.skills && user.skills.length' class="table-responsive">
							<table class="table table-striped mb-0" :class='$colorScheme.table'>
								<thead>
									<tr>
										<th scope="col">Skill</th>
										<th scope="col">Level</th>
										<th scope="col" class='text-center'>Years</th>
										<th scope="col" class='text-end'>Actions</th>
									</tr>
								</thead>
								<tbody>
									<tr v-for="skill in user.skills" :key="skill.id" class='context'>
										<td>{{ skill.skill_label }}</td>
										<td>{{ skill.levelLabel }}</td>
										<td class='text-center'>{{ skill.years }}</td>
										<td class='text-end'>
											<div class='context-actions hstack gap-1 justify-content-end'>
												<button class='btn btn-unstyled px-1 rounded' v-on:click="editSkill(skill)"><i class="bi-pencil-fill" title='Edit skill'></i></button>
												<button class='btn btn-unstyled px-1 rounded' v-on:click="confirmDelete('user.skill', skill)"><i class="bi-trash-fill" title='Delete skill'></i></button>
											</div>
										</td>
									</tr>
								</tbody>
							</table>
						</div>
						<div v-else class='fs-3 fw-light text-muted text-center p-4'>No skills</div>
					</div>
				</div>
				<div class="card shadow mt-4" :class='$colorScheme.card'>
					<div class='card-header'>
						<div class="d-flex flex-wrap justify-content-between align-items-center">
							<h3 class="h3 mb-0">Dates not available</h3>
							<button class="btn btn-primary gradient" v-on:click="editReservation()">Add</button>
						</div>
					</div>
					<div class='card-body'>
						<table class="table table-striped table-stack-mobile mb-0" :class='$colorScheme.table' v-if="reservations.length">
							<thead>
								<tr>
									<th scope="col">Description</th>
									<th scope="col">Time</th>
									<th scope="col" class='text-center'>Workload</th>
									<th scope="col" class='text-end'>Actions</th>
								</tr>
							</thead>
							<tbody>
								<tr v-for="reservation in reservations" :key="reservation.id" class='context'>
									<td data-label='Description'><div class='table-stack-mobile-cell'>{{ reservation.description }}</div></td>
									<td data-label='Time'><div class='table-stack-mobile-cell'>
										<div>
											<time :datetime='reservation.begin_time.toISOString()'>
												{{ reservation.begin_time.toLocaleDateString() }}
											</time>
											<template v-if='reservation.end_time'>
												<span>&nbsp;&mdash; </span>
												<time :datetime='reservation.end_time.toISOString()'>
													{{ reservation.end_time.toLocaleDateString() }}
												</time>
											</template>
										</div>
									</div></td>
									<td class='text-center' data-label='Workload'><div class='table-stack-mobile-cell'>{{ reservation.percentage }}%</div></td>
									<td class='text-end' data-label='Actions'><div class='table-stack-mobile-cell'>
										<div class='context-actions hstack gap-1 justify-content-end'>
											<button class='btn btn-unstyled px-1 rounded' v-on:click="editReservation(reservation)"><i class="bi-pencil-fill" title='Edit reservation'></i></button>
											<button class='btn btn-unstyled px-1 rounded' v-on:click="confirmDelete('user.reservation', reservation)"><i class="bi-trash-fill" title='Delete reservation'></i></button>
										</div>
									</div></td>
								</tr>
							</tbody>
						</table>
						<div v-else class='fs-3 fw-light text-muted text-center p-4'>No reservations</div>
					</div>
				</div>
				<div v-if='$store.state.loggeduser.isadmin' class="card shadow mt-4" :class='$colorScheme.card'>
					<div class='card-header'>
						<h3 class="h3 mb-0">Projects matching the user's skills</h3>
					</div>
					<div class='card-body'>
						<VMatchesForUser v-if='matches.length' :user='user' :matches='matches' />
						<div v-else class='fs-3 fw-light text-muted text-center p-4'>No matches</div>
					</div>
				</div>
			</div>
		</div>
	</div>
</template>

<script>
	import FormUserInfo from '@forms/FormUserInfo.vue'
	import FormUserSkill from '@forms/FormUserSkill.vue'
	import FormUserReservation from '@forms/FormUserReservation.vue'
	import VMatchesForUser from '@components/VMatchesForUser.vue'

	export default {
		name: 'UserProfile',

		components: {
			VMatchesForUser,
		},

		data() {
			return {
				user: {},
				reservations: [],
				files: [],
				newFiles: [],
				sending: false,
			}
		},

		computed: {
			submitLabel() {
				return this.sending ? 'Uploading' : 'Upload files'
			},

			matches() {
				return this.$store.state.projects.filter(project => {
					return project.matches.some(match =>
						match.user_id == this.user.id
					)
				})
			},
		},

		watch: {
			$route(to, from) {
				if (to.name == from.name) this.$options.mounted.call(this)
			},
		},

		mounted() {
			this.getUser()
			this.getReservations()
			this.getUploads()
		},

		methods: {
			async getUser() {
				// Update matches
				if (this.$store.state.loggeduser.isadmin) this.$store.dispatch('getProjects')

				const promises = [ this.$api.users.get({ id: this.$route.params.id }) ]

				if (!this.$store.state.skillLevels.length) promises.push(this.$store.dispatch('getSkillLevels'))

				const [ user ] = await Promise.all(promises)

				this.user = user

				this.user.skills.forEach(skill => {
					skill.levelLabel = this.$store.state.skillLevels.find(({ id }) => id == skill.skillscopelevel_id).label
				})
			},

			async getReservations() {
				this.reservations = await this.$api.users.reservations.get({ id: this.$route.params.id })
			},

			async getUploads() {
				const files = await this.$api.users.files.get({ user_id: this.$route.params.id })
				this.files = files
			},

			async saveFiles() {
				this.sending = true

				const success = await this.$api.users.files.save({
					user_id: this.user.id,
					files: this.newFiles,
				})

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

				this.sending = false
			},

			async setCV(id) {
				const value = this.user.main_upload_id == id ? null : id

				const data = await this.$api.users.save({
					...this.user,
					main_upload_id: value,
				})

				if (data) this.user.main_upload_id = value
			},

			downloadUrl(id) {
				return this.$api.users.files.get({ id })
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
					title: props.id ? `Edit dates not available: ${props.description}` : 'Add dates not available',
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
			},

			home() {
				this.$router.push({name: 'user-home'})
			}
		},
	}
</script>
