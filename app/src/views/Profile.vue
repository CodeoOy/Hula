<template>
	<div class="container">
		<div class="row gx-4">
			<div class="col-md-4">
				Profile here
				{{ message }}
			</div>
			<div class="col-md-8">
				<ResultsLeads />
			</div>
		</div>
	</div>
</template>

<script>
	import ResultsLeads from '../components/ResultsLeads.vue'
	export default {
		name: 'Profile',
		components: {
			'ResultsLeads': ResultsLeads
		},
		data() {
			return {
				message: "Moro"
			}
		}
	}
</script>