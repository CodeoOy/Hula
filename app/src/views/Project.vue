<template>
	<div class='container'>
		<div class='row gx-4' v-if='project'>
			<div class='col-md-4'>
				<div class='card shadow context' :class='$colorScheme.card'>
					<div class='card-header d-flex align-items-center'>
						<h1 class="h3 mb-0 flex-grow-1">
							{{ project.name }}
						</h1>
						<i v-if='project.is_hidden' class="bi-eye-slash-fill ms-3 fs-3 lh-1"></i>
						<button class='btn btn-unstyled' @click='setFavorite'>
							<i :class='favoriteClass' class="ms-3 fs-3 lh-1"></i>
						</button>
					</div>
					<div class='card-body'>
						<p v-if="project.description" class='mb-0'>{{ project.description }}</p>
						<div v-if='isAdmin' class='context-actions text-end'>
							<button class='btn btn-unstyled' v-on:click='editProject(project)'><i class='bi-pencil-fill me-2'></i></button>
							<button class='btn btn-unstyled' v-on:click='confirmDelete("project", project)'><i class='bi-trash-fill'></i></button>
						</div>
					</div>
				</div>
			</div>
			<div class='mt-4 mt-md-0 col-md-8'>
				<div class='card shadow' :class='$colorScheme.card'>
					<div class='card-header'>
						<div class='d-flex flex-wrap justify-content-between align-items-center'>
							<h2 class='h3 mb-0'>Roles</h2>
							<button v-if='isAdmin' class='btn btn-primary gradient' v-on:click='editNeed()'>Add role</button>
						</div>
					</div>
					<div class='card-body'>
						<ul v-if='project.needs.length' class='list-group list-group-flush list-group-transparent mx-n3 my-n2'>
							<li class='list-group-item' v-for='need in project.needs' :key='need.id'>
								<div class='context d-flex flex-wrap justify-content-between align-items-baseline mb-3'>
									<div>
										<h3 class='h5'>{{ need.label }}</h3>
										<div>
											<time :datetime='need.begin_time.toISOString()'>
												{{ need.begin_time.toLocaleDateString() }}
											</time>
											<template v-if='need.end_time'>
												<span>&nbsp;&mdash; </span>
												<time :datetime='need.end_time.toISOString()'>
													{{ need.end_time.toLocaleDateString() }}
												</time>
											</template>
										</div>
										<div>{{ formatPositions(need.count_of_users) }} at workload of {{ need.percentage }}%</div>
									</div>
									<div v-if='isAdmin' class='context-actions'>
										<button class='btn btn-unstyled' v-on:click='editSkill({ need })'><i class='bi-plus-circle-fill me-2'></i></button>
										<button class='btn btn-unstyled' v-on:click='editNeed(need)'><i class='bi-pencil-fill me-2'></i></button>
										<button class='btn btn-unstyled' v-on:click='confirmDelete("need", need)'><i class='bi-trash-fill me-2'></i></button>
									</div>
								</div>
								<div>
									<table class="table table-striped mb-0 table-stack-mobile" :class='$colorScheme.table'>
										<thead>
											<tr>
												<th scope='col'>Skill</th>
												<th scope='col' class='text-center'>Mandatory</th>
												<th scope='col' class='text-center'>Min level</th>
												<th scope='col' class='text-center'>Min years</th>
												<th scope='col' class='text-center'>Max years</th>
												<th v-if='isAdmin' scope='col' class='text-end'>Actions</th>
											</tr>
										</thead>
										<tbody>
											<tr v-for="skill in need.skills" :key="skill.id" class='context'>
												<td data-label='Skill'><div class='table-stack-mobile-cell'>{{ skill.skill_label }}</div></td>
												<td class='text-center' data-label='Mandatory'><div class='table-stack-mobile-cell'>{{ skill.mandatory }}</div></td>
												<td class='text-center' data-label='Min level'><div class='table-stack-mobile-cell'>{{ skill.skillscopelevel_label }}</div></td>
												<td class='text-center' data-label='Min years'><div class='table-stack-mobile-cell'>{{ skill.min_years }}</div></td>
												<td class='text-center' data-label='Max years'><div class='table-stack-mobile-cell'>{{ skill.max_years }}</div></td>
												<td v-if='isAdmin' class='text-end' data-label='Actions'><div class='table-stack-mobile-cell'>
													<div class='context-actions'>
														<button class='btn btn-unstyled' v-on:click='editSkill({ need, skill })'><i class='bi-pencil-fill me-2'></i></button>
														<button class='btn btn-unstyled' v-on:click='confirmDelete("need.skill", skill)'><i class='bi-trash-fill'></i></button>
													</div>
												</div></td>
											</tr>
										</tbody>
									</table>
								</div>
							</li>
						</ul>
						<div v-else class='fs-3 fw-light text-muted text-center p-4'>No roles</div>
					</div>
				</div>
				<div v-if='isAdmin' class="card shadow mt-4" :class='$colorScheme.card'>
					<div class='card-header'>
						<h2 class="h3 mb-0">Developers matching the roles</h2>
					</div>
					<div class='card-body'>
						<VMatchesForProject v-if='matches' :project='project' :matches='matches' />
						<div v-else class='fs-3 fw-light text-muted text-center p-4'>No matches</div>
					</div>
				</div>
			</div>
		</div>
	</div>
</template>

<script>
	import FormProject from '../forms/FormProject.vue'
	import FormProjectNeed from '../forms/FormProjectNeed.vue'
	import FormProjectNeedSkill from '../forms/FormProjectNeedSkill.vue'
	import VMatchesForProject from '../components/VMatchesForProject.vue'

	export default {
		name: 'Project',

		components: {
			VMatchesForProject,
		},

		data() {
			return {
				matches: null,
				is_favorite: false,
			}
		},

		computed: {
			isAdmin() {
				return this.$store.state.loggeduser.isadmin
			},

			project() {
				return this.$store.state.chosenproject
			},

			favoriteClass() {
				return this.is_favorite
					? ['bi-star-fill', 'text-yellow']
					: ['bi-star']
			}
		},

		async mounted() {
			this.$store.dispatch('setChosenProject', this.$route.params.id)
			if (this.$store.state.loggeduser.isadmin) this.matches = await this.$api.matches.get(this.$route.params.id)
		},

		methods: {
			formatPositions(nr) {
				return `${nr} position${nr == 1 ? '' : 's'}`
			},

			async editProject(props = {}) {
				const result = await this.$modal({
					title: 'Edit project',
					component: FormProject,
					props,
				})
				if (result) this.$store.dispatch('setChosenProject', this.$route.params.id)
			},

			setFavorite() {
				this.is_favorite = !this.is_favorite
			},

			async editNeed(props = {}) {
				const result = await this.$modal({
					title: props.id ? 'Edit role' : 'Add role',
					component: FormProjectNeed,
					props: { ...props, project_id: this.$route.params.id },
				})
				if (result) this.$store.dispatch('setChosenProject', this.$route.params.id)
			},

			async editSkill({ need = {}, skill = {} } = {}) {
				const result = await this.$modal({
					title: skill.id ? `Edit skill: ${skill.skill_label}` : 'Add Skill',
					component: FormProjectNeedSkill,
					props: {
						...skill,
						projectneed_id: need.id,
						disabledSkills: need.skills.map(skill => skill.skill_id),
					},
				})
				if (result) this.$store.dispatch('setChosenProject', this.$route.params.id)
			},

			async confirmDelete(type, data) {
				const success = await this.$confirm.delete(type, data)

				if (success) {
					switch (type) {
						case 'project':
							this.$store.dispatch('getProjects')
							this.$router.push({ name: 'admin-projects' })
							break
						
						case 'need':
						case 'need.skill':
							this.$store.dispatch('setChosenProject', this.$route.params.id)
							break
					}
				}
			},
		},
	}
</script>
