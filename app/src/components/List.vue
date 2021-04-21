<template>
	<div class="row">
		<div class="col-md-4">
			<div class="p-3 rounded-2 content-box bg-dark text-light">
				<ul class="nav nav-tabs">
					<li class="nav-item">
						<a class="nav-link active" aria-current="page" href="#" v-on:click="tabtoggle = true">Find a pro</a>
					</li>
					<li class="nav-item">
						<a class="nav-link" href="#" v-on:click="tabtoggle = false">Find a lead</a>
					</li>
				</ul>
				<FindLead v-if="tabtoggle == false" />
				<FindPro v-else />
			</div>
		</div>
		<div class="col-md-8">
			<ResultsLeads v-if="tabtoggle == false" />
			<ResultsPros v-else />
		</div>
		<ol>
			<li v-for="item in items" :key="item.pid">
				{{ item.name }}
			</li>
		</ol>
		<ul>
			<li v-for="user in users" :key="user.email">
				<a href="#" v-on:click="getUserData(user.uid)">{{user.email}}</a>
			</li>
		</ul>
	</div>
</template>

<script>
	import FindLead from './FindLead.vue'
	import FindPro from './FindPro.vue'
	import ResultsLeads from './ResultsLeads.vue'
	import ResultsPros from './ResultsPros.vue'  
	export default {
		name: 'List',
		components: {
			'FindPro': FindPro,
			'FindLead': FindLead,
			'ResultsLeads': ResultsLeads,
			'ResultsPros': ResultsPros
		},
		data() {
			return {
				tabtoggle: true
			}
		},
		methods: {
			toggleTab () {

			}
		}
	}
</script>