<template>
	<div>
		<VModal v-if='modal' ref='modal' :showAtStart='true' :modalTitle="modal.title" v-on:modal-hidden="resetModal">
			<component 
				:is='modal.component' 
				v-bind='modal.props'
				v-on:form-sent="closeModal"
			/>
		</VModal>
		<div class="d-sm-flex flex-row justify-content-between align-items-start">
			<h2 class="h2">Projects</h2>
			<div>
				<VAutoComplete
					v-if="this.$store.state.projects" 
					:suggestions="this.$store.state.projects" 
					:selection="projectName" 
					:placeholder="'filter projects'"
					:dropdown="false"
					:filterProperties="'name'"
					v-on:auto-complete="autoCompleteAction"
				></VAutoComplete>
				<button class="btn btn-gradient" v-on:click="newProject()">New project</button>
			</div>
		</div>
		<transition name="fadeHeight">
			<div class="table-responsive">
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
							<td>
								<router-link :to="{ name: 'page-project', params: { id: project.id }}">{{ project.name }}</router-link>
							</td>
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
									v-on:click.prevent="showMatch(project, match)"
								><VAvatar :user_id="match.user_id" :firstname="match.first_name" :lastname="match.last_name" />
								</a>
							</td>
							<td>
								<a href="#" v-on:click.prevent="deleteProject(project)">
									<i class="bi-trash-fill me-2"></i>
								</a>
							</td>
						</tr>
					</tbody>
				</table>
			</div>
		</transition>
	</div>
</template>

<script>
	import VModal from '../components/VModal.vue'
	import MatchContent from '../components/MatchContent.vue'
	import FormProject from '../forms/FormProject.vue'
	import FormConfirmAction from '../forms/FormConfirmAction.vue'
	import VAutoComplete from '../components/VAutoComplete.vue'
	import VAvatar from '../components/VAvatar.vue'

	export default {
		name: 'AdminListProjects',

		data () {
			return {
				modal: null,
				projectName: '',
				filteredProjects: [],
				projects: this.$store.state.projects
			}
		},

		components: {
			VModal,
			MatchContent,
			VAvatar,
			FormProject,
			FormConfirmAction,
			VAutoComplete,
		},

		methods: {
			resetModal() {
				this.modal = null
			},

			closeModal() {
				this.$refs.modal.hide()
				this.$store.dispatch('getProjects')
			},

			autoCompleteAction(value) {
				this.filteredProjects = value
			},

			newProject() {
				this.modal = {
					title: 'New project',
					component: 'FormProject',
					props: {
						chosenProject: {},
						url: '/api/projects',
						method: 'POST',
					}
				}
			},

			showMatch(project, match) {
				this.modal = {
					title: 'Match',
					component: 'MatchContent',
					props: {
						chosenMatch: match,
						chosenProject: project,
					},
				}
			},

			deleteProject(project) {
				this.modal = {
					title: `Delete ${project.name}?`,
					component: 'FormConfirmAction',
					props: {
						url: `/api/projects/${project.id}`,
						method: 'DELETE'
					}
				}
			},
		},

		activated() {
			this.$store.dispatch('getProjects')
		},
	}
</script>
