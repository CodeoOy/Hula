<template>
	<div class="row gx-4">
		<div class="col-md-4">
			<ul class="nav nav-tabs nav-dark">
				<li class="nav-item">
					<a class="nav-link" v-bind:class="{ active: tabToggle }" aria-current="page" href="#" v-on:click="tabToggle = true">Find a pro</a>
				</li>
				<li class="nav-item">
					<a class="nav-link" v-bind:class="{ active: !tabToggle }" href="#" v-on:click="tabToggle = false">Find a lead</a>
				</li>
			</ul>
			<div class="p-3 rounded-2 content-box bg-dark text-light">
				<FindLead v-on:leadsfetched="passLeads" v-if="tabToggle == false" />
				<FindPro v-on:usersfetched="passUsers" v-else />
			</div>
		</div>
		<div class="col-md-8">
			<!-- <div class="p-3 mb-4 rounded-2 content-box bg-dark text-light">
				<h2>Matches from teh algorithm</h2>
				<FeatMatches />
			</div> -->
			<ResultsLeads :leads='leadData' v-if="tabToggle == false" />
			<ResultsPros :project='projectData' v-else />
		</div>
	</div>
</template>

<script>
	import FindLead from './FindLead.vue'
	import FindPro from './FindPro.vue'
	import ResultsLeads from './ResultsLeads.vue'
	import ResultsPros from './ResultsPros.vue'
	import FeatMatches from './FeatMatches.vue'
	export default {
		name: 'AdminList',
		components: {
			FindPro,
			FindLead,
			ResultsLeads,
			ResultsPros,
			FeatMatches
		},
		data() {
			return {
				tabToggle: true,
				projectData: {},
				leadData: {},
			}
		},
		methods: {
			passUsers(value) {
				this.projectData = value
			},
			passLeads(value) {
				this.leadData = value
			}
		}
	}
</script>