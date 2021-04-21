<template>
	<div class="p-3 rounded-2 content-box bg-dark text-light">
		<h2>Projects</h2>
		<ul>
			<li v-for="lead in leads" :key="lead.pid">
				<a href="#" v-on:click="getleadData(lead.pid)">{{lead.email}}</a>
			</li>
		</ul>
	</div>
</template>

<script>
	export default {
		name: 'ResultsLeads',
		data() {
			return {
				lead: {},
				leads: {},
			}
		},
		methods: {
			getleadData: function(pid) { 
				fetch('api/query', {
					method: 'POST',
					headers: {"Content-Type": "application/json"},
					body: JSON.stringify({"pid": pid})
				})
				.then((response) => response.json())
				.then(response => { 
					console.log(response);
					this.lead = response;
				})    
			}
		}
	}
</script>