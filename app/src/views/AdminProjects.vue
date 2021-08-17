<template>
	<div>
		<VModal :modalTitle="formTitle" :modalID="'Projects'" v-on:updated-modal="chosenForm = ''">
			<component 
				:is='modalComponent' 
				:chosenProject="chosenProject" 
				:chosenMatch="chosenMatch"
				:url="url"
				:method="method"
				v-on:form-sent="hideModalUpdate"
			/>
		</VModal>
		<div class="d-flex flex-row justify-content-between align-items-start">
			<h2>Projects</h2>
			<div>
				<AutoComplete
					v-if="this.$store.state.projects" 
					:suggestions="this.$store.state.projects" 
					:selection.sync="projectName" 
					:placeholder="'filter projects'"
					:dropdown="false"
					:filterProperties="'name'"
					v-on:auto-complete="autoCompleteAction"
				></AutoComplete>
				<button
					class="btn btn-gradient"
					data-bs-toggle="modal"
					data-bs-target="#hulaModalProjects"
					v-on:click="formTitle = 'New project', chosenForm = 'CreateProject', chosenProject = {}, url='/api/projects', method='POST'"
				>New project</button>
			</div>
		</div>
		<transition name="fadeHeight">
			<table class="table table-dark table-striped text-light">
				<thead>
					<tr>
						<th scope="col">Project name</th>
						<th scope="col">Skills</th>
						<th scope="col">Matches</th>
						<th scope="col">Actions</th>
					</tr>
				</thead>
				<tbody>
					<tr v-for="project in filteredProjects" :key="project.id">
						<td><router-link
							:to="{ name: 'page-project', params: { id: project.id}}"
							v-on:click="this.chooseProject(project)"
						>{{ project.name }}</router-link></td>
						<td>
							<span
								v-for="skill in project.skills" 
								:key="skill.skill_label"
								class="badge"
							>{{ skill.skill_label }}</span>
						</td>
						<td>
							<a
								v-for="match in project.matches"
								:key="match.user_id"
								href="#"
								data-bs-toggle="modal"
								data-bs-target="#hulaModalProjects"
								v-on:click="chosenForm = 'Match', chosenMatch = match, chosenProject = project, formTitle = 'Match'"
							><VAvatar :user_id="match.user_id" :firstname="match.first_name" :lastname="match.last_name" />
							</a>
						</td>
						<td>
							<a
								href="#"
								data-bs-toggle="modal"
								data-bs-target="#hulaModalProjects" 
								v-on:click="formTitle = `Delete ${project.name}?`, chosenForm = 'Delete', url = `/api/projects/${project.id}`, method = 'DELETE'"
							><i class="bi-trash-fill me-2"></i></a>
						</td>
					</tr>
				</tbody>
			</table>
		</transition>
	</div>
</template>

<script>
	import VModal from '../components/VModal.vue'
	import MatchContent from '../components/MatchContent.vue'
	import { Modal } from 'bootstrap'
	import FormProject from '../forms/FormProject.vue'
	import FormConfirmAction from '../forms/FormConfirmAction.vue'
	import AutoComplete from '../components/AutoComplete.vue'
	import VAvatar from '../components/VAvatar.vue'
	export default {
		name: 'AdminListProjects',
		data () {
			return {
				formTitle: '',
				chosenForm: '',
				chosenProject: {},
				url: '',
				method: '',
				projectName: '',
				filteredProjects: [],
				chosenMatch: {},
				projects: this.$store.state.projects
			}
		},
		components: {
			VModal,
			MatchContent,
			VAvatar,
			FormProject,
			FormConfirmAction,
			AutoComplete,
		},
		methods: {
			chooseProject(project) {
				this.$store.commit('setChosenProject', project.id)
				this.$emit('projectChosen', project.name)
			},
			deleteProject(id) {
				fetch(`/api/projects/${id}`, {
					method: 'DELETE',
					headers: {"Content-Type": "application/json"},
					credentials: 'include'
				})
				.catch(() => {
					throw new Error('Project not deleted');
				})
			},
			hideModalUpdate() {
				let modal = Modal.getInstance(document.querySelector('#hulaModalProjects'))
				modal.hide()
				this.$store.commit('getProjects')
				console.log(this.projects)
			},
			autoCompleteAction(value) {
				this.filteredProjects = value
			}
		},
		computed: {
			modalComponent() {
				const components = {
					CreateProject: FormProject,
					Delete: FormConfirmAction,
					Match: MatchContent,
				}
				return components[this.chosenForm]
			},
		},

		activated() {
			this.$store.commit('getProjects')
		},
	}
</script>
