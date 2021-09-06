<template>
	<div class="row gx-4">
		<div class="col-md-4">
			<ul class="nav nav-tabs nav-dark">
				<li class="nav-item">
					<a class="nav-link" v-bind:class="{ active: tabToggle }" aria-current="page" href="#" v-on:click="tabToggle = true">Find by project</a>
				</li>
				<li class="nav-item">
					<a class="nav-link" v-bind:class="{ active: !tabToggle }" href="#" v-on:click="tabToggle = false">Find by developer</a>
				</li>
			</ul>
			<div class="p-3 rounded-2 content-box bg-dark text-light">
				<FindLead v-on:leadsfetched="passLeads" v-if="tabToggle == false" />
				<FindPro v-on:matchesfetched="passMatches" v-else />
			</div>
		</div>
		<div class="col-md-8">
			<div v-if='results' class="p-3 rounded-2 content-box bg-dark text-light">
				<ResultsLeads v-bind='leadData' v-if="tabToggle == false" />
				<div v-else>
					<h2 v-if="matchesData.project">Developers for {{ matchesData.project.name }}</h2>
					<p>Project skills: <span class="badge badge-skill me-2" v-for="skill in matchesData.project.skills" :key="skill.skill_id">{{ skill.skill_label }}</span></p>
					<ResultsPros :project='matchesData.project' :matches='matchesData.matches' />
				</div>
			</div>
		</div>
	</div>
</template>

<script>
	import FindLead from './FindLead.vue'
	import FindPro from './FindPro.vue'
	import ResultsLeads from './ResultsLeads.vue'
	import ResultsPros from './ResultsPros.vue'
	export default {
		name: 'AdminList',
		components: {
			FindPro,
			FindLead,
			ResultsLeads,
			ResultsPros,
		},
		data() {
			return {
				tabToggle: true,
				matchesData: null,
				leadData: null,
			}
		},
		computed: {
			results() {
				return this.tabToggle ? this.matchesData : this.leadData
			},
		},
		methods: {
			passMatches(value) {
				this.matchesData = value
			},
			passLeads(value) {
				this.leadData = value
			}
		}
	}
</script>