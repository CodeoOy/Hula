<template>
	<div>
		<VModal :modalTitle="formTitle" :modalID="'Projects'" v-on:updated-modal="chosenForm = ''">
			<component 
				:is='modalComponent' 
				:chosenProject="chosenProject" 
				:url="url"
				:method="method"
				v-on:form-sent="hideModalUpdate"
			/>
		</VModal>
		<div class="d-flex flex-row justify-content-between align-items-start">
			<h2>Projects</h2>
			<div>
				<AutoComplete
					v-if="this.$store.state.projects.length" 
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
							<p>Too expensive?</p>
						</td>
						<td>
							<p>Matches here</p>
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
	import { Modal } from 'bootstrap'
	import FormProject from '../forms/FormProject.vue'
	import FormConfirmAction from '../forms/FormConfirmAction.vue'
	import AutoComplete from '../components/AutoComplete.vue'
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
				filteredProjects: []
			}
		},
		components: {
			VModal,
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
				this.$store.commit('getProjects')
				let modal = Modal.getInstance(document.querySelector('#hulaModalProjects'))
				modal.hide()
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
				}
				return components[this.chosenForm]
			},
		},
		mounted() {
			this.$store.commit('getProjects')
		},
	}
</script>