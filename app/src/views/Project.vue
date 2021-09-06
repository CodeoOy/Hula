<template>
	<div class='container mt-4' ref='lol'>
		<div class='row gx-4' v-if='project'>
			<div class='col-md-4'>
				<div class='p-3 mb-4 rounded-2 content-box bg-dark text-light'>
					<h2 class="h2">
						{{ project.name }}
						<i v-if='project.is_hidden' class="bi-eye-slash-fill ms-3 float-end"></i>
					</h2>
					<p v-if="project.description">{{ project.description }}</p>
					<button class='btn btn-unstyled' v-on:click='editProject(project)'><i class='bi-pencil-fill me-2'></i></button>
					<button class='btn btn-unstyled' v-on:click='confirmDelete("project", project)'><i class='bi-trash-fill me-2'></i></button>
                </div>
			</div>
			<div class='col-md-8'>
				<div class='p-3 mb-4 rounded-2 content-box bg-dark text-light'>
					<div class='d-sm-flex flex-row justify-content-between align-items-start'>
						<h2 class='h2'>Needs</h2>
						<button
							class='btn btn-gradient'
							v-on:click='editNeed()'
						>Add need</button>
					</div>
					<div class='mt-3' v-for='need in project.needs' :key='need.id'>
						<hr />
						<div class='d-sm-flex flex-row justify-content-between align-items-baseline mb-3'>
							<div>
								<h5 class='h5'>{{ need.label }}</h5>
								<p>{{ need.count_of_users}} from {{ need.begin_time }} at percentage: {{ need.percentage}}</p>
							</div>
							<div>
								<button class='btn btn-unstyled' v-on:click='editSkill({ need })'><i class='bi-plus-circle-fill me-2'></i></button>
								<button class='btn btn-unstyled' v-on:click='editNeed(need)'><i class='bi-pencil-fill me-2'></i></button>
								<button class='btn btn-unstyled' v-on:click='confirmDelete("need", need)'><i class='bi-trash-fill me-2'></i></button>
							</div>
						</div>
						<div class="table-responsive">
							<table class="table table-dark table-striped text-light mb-4">
								<thead>
									<tr>
										<th scope='col'>Skill</th>
										<th scope='col'>Mandatory</th>
										<th scope='col'>Min level</th>
										<th scope='col'>Min years</th>
										<th scope='col'>Max years</th>
										<th scope='col'>Actions</th>
									</tr>
								</thead>
								<tbody>
									<tr v-for="skill in need.skills" :key="skill.id">
										<td>{{ skill.skill_label }}</td>
										<td>{{ skill.mandatory }}</td>
										<td>{{ skill.skillscopelevel_label }}</td>
										<td>{{ skill.min_years }}</td>
										<td>{{ skill.max_years }}</td>
										<td class='hoverable-td'>
											<button class='btn btn-unstyled' v-on:click='editSkill({ need, skill })'><i class='bi-pencil-fill me-2'></i></button>
											<button class='btn btn-unstyled' v-on:click='confirmDelete("need.skill", skill)'><i class='bi-trash-fill me-2'></i></button>
										</td>
									</tr>
								</tbody>
							</table>
						</div>
					</div>
				</div>
				<div v-if='matches' class="p-3 mb-4 rounded-2 content-box bg-dark text-light">
					<h2 class="h2">Pros matching the needs</h2>
					<ResultsPros :project='project' :matches='matches' />
				</div>
			</div>
		</div>
	</div>
</template>

<script>
	import FormProject from '../forms/FormProject.vue'
	import FormProjectNeed from '../forms/FormProjectNeed.vue'
	import FormProjectNeedSkill from '../forms/FormProjectNeedSkill.vue'
	import ResultsPros from '../components/ResultsPros.vue'

	export default {
		name: 'Project',

		components: {
			ResultsPros,
		},

		data() {
			return {
				matches: null,
			}
		},

		computed: {
			project() {
				return this.$store.state.chosenproject
			},
		},

		async mounted() {
			this.$store.dispatch('setChosenProject', this.$route.params.id)
			if (this.$store.state.loggeduser.isadmin) this.matches = await this.$api.matches.get(this.$route.params.id)
		},

		methods: {
			async editProject(props = {}) {
				const result = await this.$modal({
					title: 'Edit project',
					component: FormProject,
					props,
				})
				if (result) this.$store.dispatch('setChosenProject', this.$route.params.id)
			},

			async editNeed(props = {}) {
				const result = await this.$modal({
					title: props.id ? 'Edit need' : 'Add need',
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
