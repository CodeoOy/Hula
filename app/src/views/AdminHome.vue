<template>
	<div class="container-fluid mt-4">
		<div class="row gx-4">
			<div class="col-md-4">
				<ul class="nav nav-tabs">
					<li class="nav-item">
						<button class="nav-link" v-bind:class="{ active: tabToggle }" v-on:click="tabToggle = true">Find by project</button>
					</li>
					<li class="nav-item">
						<button class="nav-link" v-bind:class="{ active: !tabToggle }" v-on:click="tabToggle = false">Find by developer</button>
					</li>
				</ul>
				<div class="p-3 rounded shadow bg-dark text-light">
					<keep-alive>
						<VFilterList
							v-if='projects.length && tabToggle'
							:items='projects'
							placeholder='Filter projects'
							@select="onSelectProject"
						/>
						<VFilterList
							v-else-if="!tabToggle"
							:items='users'
							placeholder='Filter developers'
							@select='onSelectUser'
						/>
					</keep-alive>
				</div>
			</div>
			<div class="col-md-8">
				<div v-if='results' class="p-3 rounded shadow bg-dark text-light">
					<ResultsLeads v-bind='leadData' v-if="tabToggle == false" />
					<div v-else>
						<h2 v-if="matchesData.project">Developers for {{ matchesData.project.name }}</h2>
						<p>Project skills: <span class="badge badge-skill me-2" v-for="skill in matchesData.project.skills" :key="skill.skill_id">{{ skill.skill_label }}</span></p>
						<ResultsPros :project='matchesData.project' :matches='matchesData.matches' />
					</div>
				</div>
			</div>
		</div>
	</div>
</template>

<script>
	import VFilterList from '../components/VFilterList.vue'
	import ResultsLeads from '../components/ResultsLeads.vue'
	import ResultsPros from '../components/ResultsPros.vue'

	export default {
		name: 'AdminHome',

		components: {
			VFilterList,
			ResultsLeads,
			ResultsPros,
		},

		data() {
			return {
				tabToggle: true,
				matchesData: null,
				leadData: null,
				users: [],
			}
		},

		computed: {
			projects() {
				return this.$store.state.projects
			},

			results() {
				return this.tabToggle ? this.matchesData : this.leadData
			},
		},

		async mounted() {
			if (!this.$store.state.projects.length) this.$store.dispatch('getProjects')	
			this.users = await this.$api.users.get()
			this.users.forEach(user => user.name = `${user.firstname} ${user.lastname}`)
		},

		methods: {
			onSelectUser(user) {
				const projects = this.$store.state.projects.filter(project => {
					return project.matches.some(match => {
						return match.user_id === user.id
					})
				})
				this.leadData = { user, projects }
			},

			async onSelectProject(project) {
				if (!project) return null
				const matches = await this.$api.matches.get(project.id)
				this.matchesData = matches ? { project, matches } : null
			},
		},
	}
</script>