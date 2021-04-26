<template>
	<div class="row gx-4">
		<div class="col-md-4">
			<ul class="nav nav-tabs nav-dark">
				<li class="nav-item">
					<a class="nav-link" v-bind:class="{ active: tabtoggle }" aria-current="page" href="#" v-on:click="tabtoggle = true">Find a pro</a>
				</li>
				<li class="nav-item">
					<a class="nav-link" v-bind:class="{ active: !tabtoggle }" href="#" v-on:click="tabtoggle = false">Find a lead</a>
				</li>
			</ul>
			<div class="p-3 rounded-2 content-box bg-dark text-light">
				<FindLead v-on:datafetched="passData" v-if="tabtoggle == false" />
				<FindPro v-on:datafetched="passData" v-else />
			</div>
		</div>
		<div class="col-md-8">
			<div class="p-3 mb-4 rounded-2 content-box bg-dark text-light">
				<h2>Very good matches</h2>
				<p>In this box we can show good matches that were not actively searched but found by algorithm.</p>
				<FeatMatches />
			</div>
			<ResultsLeads :leads='currentdata' v-if="tabtoggle == false" />
			<ResultsPros :users='currentdata' v-else />
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
		name: 'List',
		components: {
			'FindPro': FindPro,
			'FindLead': FindLead,
			'ResultsLeads': ResultsLeads,
			'ResultsPros': ResultsPros,
			'FeatMatches': FeatMatches
		},
		data() {
			return {
				tabtoggle: true,
				currentdata: {}
			}
		},
		methods: {
			passData (value) {
				this.currentdata = value
			}
		}
	}
</script>