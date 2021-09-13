<template>
	<div class='card shadow' :class='$colorScheme.card'>
		<div class='card-header'>
			<div class="d-flex justify-content-between align-items-center flex-wrap">
				<h1 class="h3 flex-grow-1 mb-0">Projects</h1>
				<div class='input-group order-last mt-3 order-md-0 me-md-2 w-md-auto mt-md-0'>
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
								<label for='inactive'>Exclude inactive</label>
								<input v-model='filters.hideInactive' type='checkbox' class='form-check-input' id='inactive' />
							</div>
						</li>
						<li class='px-2'>
							<div class='form-check'>
								<label for='hidden'>Exclude hidden</label>
								<input v-model='filters.hideHidden' type='checkbox' class='form-check-input' id='hidden' />
							</div>
						</li>
					</ul>
				</div>
				<button class="btn btn-primary gradient flex-shrink-0" v-on:click="newProject()">New project</button>
			</div>
		</div>
		<div class='card-body'>
			<div v-if="filteredProjects.length" class="table-responsive">
				<table class="table table-striped" :class='$colorScheme.table'>
					<thead>
						<tr>
							<th scope="col">Project name</th>
							<th scope="col">Skills</th>
							<th scope="col">Matches</th>
							<th scope="col" class='text-end'>Actions</th>
						</tr>
					</thead>
					<tbody>
						<tr v-for="project in filteredProjects" :key="project.id">
							<td class='d-flex align-items-center'>
								<div class='flex-grow-1'>
									<router-link :to="{ name: 'project', params: { id: project.id }}">
										{{ project.name }}
									</router-link>
								</div>
								<i v-if='!project.is_active' class="bi-clock-fill ms-2"></i>
								<i v-if='project.is_hidden' class="bi-eye-slash-fill ms-2"></i>
							</td>
							<td>
								<span
									v-for="skill in project.skills" 
									:key="skill.skill_label"
									class="badge badge-skill me-2"
								>{{ skill.skill_label }}</span>
							</td>
							<td>
								<button
									v-for="match in project.matches"
									:key="match.user_id"
									v-on:click="showMatch(project, match)"
									class='btn btn-unstyled'
								><VAvatar :user_id="match.user_id" :firstname="match.first_name" :lastname="match.last_name" :favorite='match.is_favorite' />
								</button>
							</td>
							<td class='text-end'>
								<button class='btn btn-unstyled' v-on:click="confirmDelete(project)"><i class="bi-trash-fill me-2"></i></button>
							</td>
						</tr>
					</tbody>
				</table>
			</div>
			<div v-else>
				<div class='fs-3 fw-light text-muted text-center p-4'>{{ noProjectsMessage }}</div>
			</div>
		</div>
	</div>
</template>

<script>
	import MatchContent from '../components/MatchContent.vue'
	import FormProject from '../forms/FormProject.vue'
	import VAutoComplete from '../components/VAutoComplete.vue'
	import VAvatar from '../components/VAvatar.vue'

	export default {
		name: 'AdminListProjects',

		data () {
			return {
				autoCompletedProjects: [],
				filters: {
					hideInactive: false,
					hideHidden: false,
				},
				matchSkills: {},
			}
		},

		components: {
			MatchContent,
			VAvatar,
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
					.filter(project => project.is_active ? true : !this.filters.hideInactive)
					.filter(project => project.is_hidden ? !this.filters.hideHidden : true)
			},

			noProjectsMessage() {
				return this.projects.length
					? 'No projects matching the filter'
					: 'No projects'
			}
		},

		methods: {
			autoCompleteAction(value) {
				this.autoCompletedProjects = value
			},

			async newProject() {
				const result = await this.$modal({
					title: 'New project',
					component: FormProject,
				})

				if (result) this.$store.dispatch('getProjects')
			},

			async showMatch(project, match) {
				if (!(project.id in this.matchSkills)) {
					this.matchSkills[project.id] = await this.$api.matches.get(project.id)
				}

				this.$modal({
					title: 'Match',
					component: MatchContent,
					props: this.matchSkills[project.id][match.user_id],
				})
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
