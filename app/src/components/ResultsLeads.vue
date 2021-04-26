<template>
	<div class="p-3 rounded-2 content-box bg-dark text-light">
		<h2>Project search results</h2>
		<table class="table table-dark table-striped text-light">
			<thead>
				<tr>
					<th scope="col">#</th>
					<th scope="col">Project name</th>
					<th scope="col">Available</th>
				</tr>
			</thead>
			<tbody>
				<tr v-for="(lead, index) in leads" :key="lead.pid">
					<th scope="row">{{ index + 1 }}</th>
					<td>{{ lead.name }}</td>
					<td>{{ lead.available }}</td>
				</tr>
			</tbody>
		</table>
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