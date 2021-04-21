<template>
	<div class="p-3 rounded-2 content-box bg-dark text-light">
		<h2>Project search results</h2>
		<ul>
			<li v-for="lead in leads" :key="lead.pid">
				<a href="#" v-on:click="getleadData(lead.pid)">{{lead.name}}</a>
			</li>
		</ul>
	</div>
</template>

<script>
	export default {
		name: 'ResultsLeads',
		data() {
			return {
				message: "Heips"
			}
		},
		props: {
			leads: {}
		},
		methods: {
			getleadData: function(pid) { 
				fetch('api/project', {
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