<template>
	<div class="container mt-4" ref="lol">
		<VModal :modalTitle="formTitle" :modalID="'SingleProject'" v-on:modal-hidden="chosenForm = '', chosenSkill = {}">
			<component 
				:is='modalComponent'
				:chosenNeed="chosenNeed"
				:chosenSkill="chosenSkill"
				:chosenProject="chosenProject"
				:url="url"
				:method="method"
				v-on:form-sent="hideModalUpdate"
			/>
		</VModal>
		<div class="row gx-4" v-if='project'>
			<div class="col-md-4">
				<div class="p-3 mb-4 rounded-2 content-box bg-dark text-light">
					<h2 class="h2">
						{{ project.name }}
						<i v-if='project.is_hidden' class="bi-eye-slash-fill ms-3 float-end"></i>
					</h2>
					<p v-if="project.description">{{ project.description }}</p>
					<a 
						href="#"
						data-bs-toggle="modal" 
						data-bs-target="#hulaModalSingleProject" 
						v-on:click="formTitle = 'Edit project', chosenForm = 'Project', chosenProject = project, url=`/api/projects/${project.id}`, method='PUT'"
					><i class="bi-pencil-fill me-2"></i></a>
					<a 
						href="#"
						v-on:click.prevent="confirmDelete('project', project)"
					><i class="bi-trash-fill me-2"></i></a>
                </div>
			</div>
			<div class="col-md-8">
				<div class="p-3 mb-4 rounded-2 content-box bg-dark text-light">
					<div class="d-sm-flex flex-row justify-content-between align-items-start">
						<h2 class="h2">Needs</h2>
						<button
							class="btn btn-gradient"
							data-bs-toggle="modal"
							data-bs-target="#hulaModalSingleProject"
							v-on:click="formTitle = 'Add need', chosenForm = 'Need', chosenNeed = chosenNeedDefault"
						>Add need</button>
					</div>
					<div class="mt-3" v-for="need in project.needs" :key="need.id">
						<hr />
						<div class="d-sm-flex flex-row justify-content-between align-items-baseline mb-3">
							<div>
								<h5 class="h5">{{ need.label }}</h5>
								<p>{{ need.count_of_users}} from {{ need.begin_time }} at percentage: {{ need.percentage}}</p>
							</div>
							<div class="btn-group" role="group" aria-label="Need actions">
								<a 
									href="#"
									data-bs-toggle="modal" 
									data-bs-target="#hulaModalSingleProject" 
									v-on:click="chosenNeed = need, formTitle = 'New skill', chosenSkill = {}, chosenForm = 'Skill', url=`/api/projectskills`, method='POST'"
								><i class="bi-plus-circle-fill me-2"></i></a>
								<a
									href="#"
									data-bs-toggle="modal" 
									data-bs-target="#hulaModalSingleProject" 
									v-on:click="chosenNeed = need, formTitle = 'Edit need', chosenForm = 'Need'"
								><i class="bi-pencil-fill me-2"></i></a>
								<a
									href="#"
									v-on:click.prevent="confirmDelete('need', need)"
								><i class="bi-trash-fill me-2"></i></a>
							</div>
						</div>
						<div class="table-responsive">
							<table class="table table-dark table-striped text-light mb-4">
								<thead>
									<tr>
										<th scope="col">Skill</th>
										<th scope="col">Mandatory</th>
										<th scope="col">Min level</th>
										<th scope="col">Min years</th>
										<th scope="col">Max years</th>
										<th scope="col">Actions</th>
									</tr>
								</thead>
								<tbody>
									<tr v-for="skill in need.skills" :key="skill.id">
										<td>{{ skill.skill_label }}</td>
										<td>{{ skill.mandatory }}</td>
										<td>{{ skill.skillscopelevel_label }}</td>
										<td>{{ skill.min_years }}</td>
										<td>{{ skill.max_years }}</td>
										<td class="hoverable-td">
											<a 
												href="#"
												data-bs-toggle="modal"
												data-bs-target="#hulaModalSingleProject"
												v-on:click="formTitle = skill.skill_label, chosenForm = 'Skill', chosenSkill = skill, chosenNeed = need, url=`/api/projectskills/${skill.id}`, method='PUT'"
											><i class="bi-pencil-fill me-2"></i></a>
											<a
												href="#"
												v-on:click.prevent="confirmDelete('need.skill', skill)"
											><i class="bi-trash-fill me-2"></i></a>
										</td>
									</tr>
								</tbody>
							</table>
						</div>
					</div>
				</div>
				<div v-if='matches.length' class="p-3 mb-4 rounded-2 content-box bg-dark text-light">
					<h2 class="h2">Pros matching the needs</h2>
					<ResultsPros :project='project' :matches='matches' />
				</div>
			</div>
		</div>
	</div>
</template>

<script>
	import VModal from '../components/VModal.vue'
	import { Modal } from 'bootstrap'
	import FormProjectNeed from '../forms/FormProjectNeed.vue'
	import FormProjectNeedSkill from '../forms/FormProjectNeedSkill.vue'
	import FormProject from '../forms/FormProject.vue'
	import ResultsPros from '../components/ResultsPros.vue'
	export default {
		name: 'Project',
		data() {
			return {
				chosenNeed: {},
				chosenNeedDefault: {
					isNew: true,
					project_id: this.$store.state.chosenproject ? this.$store.state.chosenproject.id : undefined,
					count_of_users: Number,
					begin_time: String,
					end_time: String,
					percentage: Number,
					updated_by: this.$store.state.loggeduser.email
				},
				formTitle: '',
				chosenForm: '',
				chosenProject: {},
				chosenSkill: {},
				url: '',
				method: '',
				matches: [],
			}
		},
		components: {
			VModal,
			FormProjectNeed,
			FormProjectNeedSkill,
			FormProject,
			ResultsPros,
		},
		computed: {
			project() {
				return this.$store.state.chosenproject
			},
			modalComponent() {
				const components = {
					Need: FormProjectNeed,
					Skill: FormProjectNeedSkill,
					Project: FormProject,
				}
				return components[this.chosenForm]
			},
		},
		methods: {
			hideModalUpdate() {
				this.$store.dispatch('setChosenProject', this.$route.params.id)
				let modal = Modal.getInstance(document.querySelector('#hulaModalSingleProject'))
				modal.hide()
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
		async mounted() {
			this.$store.dispatch('setChosenProject', this.$route.params.id)
			this.matches = await this.$api.matches.get(this.$route.params.id)
		}
	}
</script>
