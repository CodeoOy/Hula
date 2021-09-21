<template>
	<div class='card shadow' :class='$colorScheme.card'>
		<div class='card-header'>
			<div class="d-flex justify-content-between align-items-center flex-wrap">
				<h1 class="h3 flex-grow-1 mb-0">Projects</h1>
				<div class='input-group order-last mt-3 order-md-0 me-md-2 w-md-auto mt-md-0'>
					<VFilter
						:items='projects'
						:props='["name", "skillLabels"]'
						placeholder='filter projects'
						@filter="filterProjects"
					/>
					<button class='btn btn-secondary' type='button' data-bs-toggle="collapse" data-bs-target="#filters" aria-expanded="false" aria-controls="filters">
						<i aria-label='Filters' class='bi bi-gear-fill'></i>
					</button>
				</div>
				<button class="btn btn-primary gradient flex-shrink-0" v-on:click="newProject()">New project</button>
			</div>
			<div class="collapse float-end" id="filters">
				<div class='mt-3 mb-2'>
					<span class='fw-bold me-3'>Include:</span>
					<div class='form-check form-check-inline'>
						<label for='inactive'>inactive</label>
						<input v-model='filters.inactive' type='checkbox' class='form-check-input' id='inactive' />
					</div>
					<div class='form-check form-check-inline'>
						<label for='hidden'>hidden</label>
						<input v-model='filters.hidden' type='checkbox' class='form-check-input' id='hidden' />
					</div>
				</div>
			</div>
		</div>
		<div class='card-body'>
			<div v-if="filteredProjects.length">
				<table class="table table-striped table-stack-mobile mb-0" :class='$colorScheme.table'>
					<thead>
						<tr>
							<th scope="col">Project name</th>
							<th scope="col">Skills</th>
							<th scope="col">Matches</th>
							<th scope="col" class='text-end'>Actions</th>
						</tr>
					</thead>
					<transition-group name='flip-list' tag='tbody' @before-leave='onBeforeTrLeave'>
						<tr v-for="project in filteredProjects" :key="project.id" class='context'>
							<td data-label='Project'>
								<div class='table-stack-mobile-cell'>
									<div class='d-flex align-items-center'>
										<div class='flex-grow-1'>
											<router-link :to="{ name: 'project', params: { id: project.id }}">
												<VHighlight :text='project.name' :pattern='highlightPattern' />
											</router-link>
										</div>
										<i v-if='!project.is_active' class="bi-clock-fill ms-2"></i>
										<i v-if='project.is_hidden' class="bi-eye-slash-fill ms-2"></i>
									</div>
								</div>
							</td>
							<td data-label='Skills'>
								<div class='table-stack-mobile-cell'>
									<div class='hstack gap-2 flex-wrap'>
										<VSkillBadge
											v-for="skill in project.skills" 
											:key="skill.skill_label"
											:label='skill.skill_label'
											:mandatory='skill.skill_mandatory'
											:percentage='skill.skill_percentage'
											:highlight='highlight(skill)'
										/>
									</div>
								</div>
							</td>
							<td data-label='Matches'>
								<div class='table-stack-mobile-cell'>
									<div class='hstack gap-1 flex-wrap'>
										<button
											v-for="match in project.matches"
											:key="match.user_id"
											v-on:click="showMatch(project, match)"
											class='btn btn-unstyled rounded-circle'
										><VAvatar
												:id="match.user_id"
												:firstName="match.first_name"
												:lastName="match.last_name"
												:favorite='match.is_favorite'
											/>
										</button>
									</div>
								</div>
							</td>
							<td class='text-end' data-label='Actions'>
								<div class='table-stack-mobile-cell'>
									<div class='context-actions'>
										<button class='btn btn-unstyled px-1 rounded' v-on:click="confirmDelete(project)"><i class="bi-trash-fill"></i></button>
									</div>
								</div>
							</td>
						</tr>
					</transition-group>
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
	import VFilter from '../components/VFilter.vue'
	import VHighlight from '../components/VHighlight.vue'
	import VAvatar from '../components/VAvatar.vue'
	import VSkillBadge from '../components/VSkillBadge.vue'
	import { onBeforeTrLeave } from '../transitions.js'

	export default {
		name: 'AdminListProjects',

		data() {
			return {
				projectsByKeyword: [],
				filters: {
					inactive: true,
					hidden: true,
				},
				matchSkills: {},
				highlightPattern: null,
			}
		},

		components: {
			VAvatar,
			VFilter,
			VHighlight,
			VSkillBadge,
		},

		computed: {
			projects() {
				return this.$store.state.projects.map(project => ({
					...project,
					skillLabels: project.skills.map(skill => skill.skill_label),
				}))
			},

			filteredProjects() {
				return this.projectsByKeyword
					.filter(project => project.is_active ? true : this.filters.inactive)
					.filter(project => project.is_hidden ? this.filters.hidden : true)
			},

			noProjectsMessage() {
				return this.projects.length
					? 'No projects matching the filter'
					: 'No projects'
			},
		},

		methods: {
			onBeforeTrLeave,

			filterProjects({ matches, pattern }) {
				this.projectsByKeyword = matches
				this.highlightPattern = pattern
			},

			highlight(skill) {
				return Boolean(this.highlightPattern && skill.skill_label.toUpperCase().match(this.highlightPattern))
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
					this.matchSkills[project.id] = await this.$api.matches.get({ id: project.id })
				}

				this.$modal({
					title: 'Match',
					component: MatchContent,
					size: 'lg',
					props: this.matchSkills[project.id][match.user_id],
				})
			},

			async confirmDelete(project) {
				const success = await this.$confirm.delete('project', project)
				if (success) this.$store.dispatch('getProjects')
			},
		},

		activated() {
			this.$store.dispatch('getProjects')
		},
	}
</script>
