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
			<button
				class="btn btn-gradient"
				data-bs-toggle="modal"
				data-bs-target="#hulaModalProjects"
				v-on:click="formTitle = 'New project', chosenForm = 'CreateProject', chosenProject = chosenProjectDefault, url='/api/projects', method='POST'"
			>New project</button>
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
					<tr v-for="project, in this.$store.state.projects" :key="project.id">
						<td><router-link
							:to="{ name: 'page-project', params: { id: project.id}}"
							v-on:click="this.chooseProject(project)"
						>{{ project.name }}</router-link></td>
						<td>
							<p v-for="need in project.needs" :key="need.id">{{ need.id }}</p>
						</td>
						<td>
							<p>Matches here</p>
						</td>
						<td>
							<a
								href="#"
								data-bs-toggle="modal"
								data-bs-target="#hulaModalProjects" 
								v-on:click="formTitle = `Delete ${project.label}?`, chosenForm = 'Delete', url = `/api/skills/${project.id}`, method = 'DELETE'"
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
	export default {
		name: 'AdminListProjects',
		data () {
			return {
				formTitle: '',
				chosenForm: '',
				chosenProject: {},
				url: '',
				method: '',
			}
		},
		components: {
			VModal,
			FormProject,
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
			},
		},
		computed: {
			modalComponent() {
				const components = {
					CreateProject: FormProject,
				}
				return components[this.chosenForm]
			}
		},
		mounted() {
			this.$store.commit('getProjects')
		},
	}
</script>