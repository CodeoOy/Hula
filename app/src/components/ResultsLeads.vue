<template>
	<div class="p-3 rounded-2 content-box bg-dark text-light">
		<h2 class="h2">Lead search results</h2>
		<div class="table-responsive">
			<table v-if="leads.length" class="table table-dark table-striped text-light">
				<thead>
					<tr>
						<th scope="col">Project</th>
						<th scope="col">Skills</th>
						<th scope="col"></th>
					</tr>
				</thead>
				<tbody>
					<tr v-for="lead in leads" :key="lead.pid">
						<th scope="row"><a :href="`app/project/${lead.id}`">{{ lead.name }}</a></th>
						<td>{{ lead.name }}</td>
						<td>{{ lead.is_hidden }}</td>
					</tr>
				</tbody>
			</table>
		</div>
	</div>
</template>

<script>
	export default {
		name: 'ResultsLeads',
		props: {
			leads: {}
		}
	}
</script>