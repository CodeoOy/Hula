<template>
	<div class="container mt-4" ref="lol">
		<VModal :modalTitle="formTitle" :modalID="'SingleProject'" v-on:updated-modal="chosenForm = ''">
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
		<div class="row gx-4">
			<div class="col-md-4">
                <div class="p-3 mb-4 rounded-2 content-box bg-dark text-light">
                	<h2>{{ project.name }}</h2>
					<a 
						href="#"
						data-bs-toggle="modal" 
						data-bs-target="#hulaModalSingleProject" 
						v-on:click="formTitle = 'Edit project', chosenForm = 'Project', chosenProject = project, url=`/api/projects/${project.id}`, method='PUT'"
					><i class="bi-pencil-fill me-2"></i></a>
					<a 
						href="#"
						data-bs-toggle="modal" 
						data-bs-target="#hulaModalSingleProject" 
						v-on:click="formTitle = 'Delete project', chosenForm = 'Delete', chosenProject = project, url=`/api/projects/${project.id}`, method='DELETE'"
					><i class="bi-trash-fill me-2"></i></a>
                </div>
			</div>
			<div class="col-md-8">
				<div class="p-3 mb-4 rounded-2 content-box bg-dark text-light">
					<div class="d-flex flex-row justify-content-between align-items-start">
						<h2>Needs</h2>
						<button
							class="btn btn-gradient"
							data-bs-toggle="modal"
							data-bs-target="#hulaModalSingleProject"
							v-on:click="formTitle = 'Add need', chosenForm = 'Need', chosenNeed = chosenNeedDefault"
						>Add need</button>
					</div>
					<div class="mt-3" v-for="need in project.needs" :key="need.id">
						<hr />
						<div class="d-flex flex-row justify-content-between align-items-baseline mb-3">
							<h5>{{ need.count_of_users}} from {{ need.begin_time }} at percentage: {{ need.percentage}}</h5>
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
									data-bs-toggle="modal"
									data-bs-target="#hulaModalSingleProject" 
									v-on:click="formTitle = `Delete ${need.id}?`, chosenForm = 'Delete', url = `/api/projectneeds/${need.id}`, method = 'DELETE'"
								><i class="bi-trash-fill me-2"></i></a>
							</div>
						</div>
						<table v-if="skills.length" class="table table-dark table-striped text-light mb-4">
							<thead>
								<tr>
									<th scope="col">Skill</th>
									<th scope="col">Min level</th>
									<th scope="col">Min years</th>
									<th scope="col">Max years</th>
									<th scope="col">Actions</th>
								</tr>
							</thead>
							<tbody>
								<tr v-for="skill in need.skills" :key="skill.id">
									<td>{{ getSkillLabel(skill.skill_id) }}</td>
									<td>{{ skill.skillscopelevel_id }}</td>
									<td>{{ skill.min_years }}</td>
									<td>{{ skill.max_years }}</td>
									<td class="hoverable-td">
										<a 
											href="#"
											data-bs-toggle="modal"
											data-bs-target="#hulaModalSingleProject"
											v-on:click="formTitle =`${getSkillLabel(skill.skill_id)}`, chosenForm = 'Skill', chosenSkill = skill, chosenNeed = need, url=`/api/projectskills/${skill.id}`, method='PUT'"
										><i class="bi-pencil-fill me-2"></i></a>
										<a
											href="#"
											data-bs-toggle="modal"
											data-bs-target="#hulaModalSingleProject" 
											v-on:click="formTitle = `Delete ${skill.id}?`, chosenForm = 'Delete', url = `/api/projectskills/${skill.id}`, method = 'DELETE'"
										><i class="bi-trash-fill me-2"></i></a>
									</td>
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
	import FormProjectNeed from '../forms/FormProjectNeed.vue'
	import FormProjectNeedSkill from '../forms/FormProjectNeedSkill.vue'
	import FormConfirmAction from '../forms/FormConfirmAction.vue'
	import FormProject from '../forms/FormProject.vue'
	export default {
		name: 'Project',
		data() {
			return {
				chosenNeed: {},
				chosenNeedDefault: {
					isNew: true,
					project_id: this.$store.state.chosenproject.id,
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
				skills: [],
			}
		},
		components: {
			VModal,
			FormProjectNeed,
			FormProjectNeedSkill,
			FormConfirmAction,
			FormProject,
		},
		computed: {
			project() {
				return this.$store.state.chosenproject
			},
			modalComponent() {
				const components = {
					Need: FormProjectNeed,
					Skill: FormProjectNeedSkill,
					Delete: FormConfirmAction,
					Project: FormProject,
				}
				return components[this.chosenForm]
			},
		},
		methods: {
			hideModalUpdate() {
				this.checkProject(this.$route.params.id)
				let modal = Modal.getInstance(document.querySelector('#hulaModalSingleProject'))
				modal.hide()
			},
			getAllSkills() {
				fetch('/api/skills', {method: 'GET'})
				.then(response => { 
					return (response.status >= 200 && response.status <= 299) ? response.json() : this.$store.commit('errorHandling', response)
				})
				.then(response => { 
					this.skills = response;
				})    
				.catch((errors) => {
					this.$store.commit('errorHandling', errors)
				})
			},
			getSkillLabel(id) {
				var returnedSkill = this.skills.find(skill => skill.id == id)
				return returnedSkill.label
			},
			checkProject(id) {
				fetch(`/api/projects/${id}`, {method: 'GET'})
				.then(response => { 
					if (!response.ok) {
						this.$router.push({name: 'page-error'})
					} else {
						this.$store.dispatch('setChosenProject', this.$route.params.id)
					}
				})
			}
		},
		mounted() {
			this.checkProject(this.$route.params.id)
			this.getAllSkills()
		}
	}
</script>