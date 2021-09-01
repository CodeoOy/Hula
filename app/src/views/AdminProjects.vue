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
			<div class='d-flex'>
				<div class='input-group me-2'>
					<VAutoComplete
						:suggestions="projects" 
						:placeholder="'filter projects'"
						:dropdown="false"
						:filterProperties="['name', 'autoCompleteSkills']"
						v-on:auto-complete="autoCompleteAction"
					/>
					<button class='btn btn-secondary dropdown-toggle' type='button' id='filtersDropdown' data-bs-toggle='dropdown' data-bs-auto-close='outside' aria-expanded='false'>
						<i aria-label='Filters' class='bi bi-gear-fill'></i>
					</button>
					<ul class='dropdown-menu dropdown-menu-end' aria-labelledby='filtersDropdown'>
						<li class='px-2'>
							<div class='form-check'>
								<label for='hidden'>Show hidden</label>
								<input v-model='filters.showHidden' type='checkbox' class='form-check-input' id='hidden' />
							</div>
						</li>
					</ul>
				</div>
				<button class="btn btn-gradient flex-shrink-0" v-on:click="newProject()">New project</button>
			</div>
		</div>
		<div class="table-responsive" v-if="filteredProjects.length">
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
							<router-link :to="{ name: 'page-project', params: { id: project.id }}">
								{{ project.name }}
								<i v-if='project.is_hidden' class="bi-eye-slash-fill ms-2 float-end"></i>
							</router-link>
						</td>
						<td>
							<span
								v-for="skill in project.skills" 
								:key="skill.skill_label"
								class="badge badge-skill me-2"
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
							<a href="#" v-on:click.prevent="confirmDelete(project)">
								<i class="bi-trash-fill me-2"></i>
							</a>
						</td>
					</tr>
				</tbody>
			</table>
		</div>
	</div>
</template>

<script>
	import VModal from '../components/VModal.vue'
	import MatchContent from '../components/MatchContent.vue'
	import FormProject from '../forms/FormProject.vue'
	import VAutoComplete from '../components/VAutoComplete.vue'
	import VAvatar from '../components/VAvatar.vue'

	export default {
		name: 'AdminListProjects',

		data () {
			return {
				modal: null,
				autoCompletedProjects: [],
				filters: {
					showHidden: true,
				},
			}
		},

		components: {
			VModal,
			MatchContent,
			VAvatar,
			FormProject,
			VAutoComplete,
		},

		computed: {
			projects() {
				return this.$store.state.projects.map(project => ({
					...project,
					autoCompleteSkills: project.skills.map(skill => skill.skill_label),
				}))
			},

			filteredProjects() {
				return this.autoCompletedProjects
					.filter(project => project.is_hidden ? this.filters.showHidden : true)
			},
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
				this.autoCompletedProjects = value
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
						projectName: project.name,
						projectID: project.id,
					},
				}
			},

			async confirmDelete(project) {
				const success = await this.$confirm.delete('project', project)
				if (success) this.$store.dispatch('getProjects')
			},
		},

		mounted() {
			if (!this.$store.state.projects.length) this.$store.dispatch('getProjects')
		},
	}
</script>
