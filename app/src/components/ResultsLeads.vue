<template>
	<div>
		<h2 class="h2">Lead search results</h2>
		<div class="table-responsive">
			<table v-if="leads.length" class="table table-dark table-striped text-light">
				<thead>
					<tr>
						<th scope="col">Project</th>
						<th scope="col">Skills</th>
					</tr>
				</thead>
				<tbody>
					<tr v-for="lead in leads" :key="lead.id">
						<td>
							<router-link :to="{ name: 'page-project', params: { id: lead.id }}">
								{{ lead.name }}
							</router-link>
							<i v-if='lead.is_hidden' class="bi-eye-slash-fill ms-2 float-end"></i>
						</td>
						<td>
							<span
								v-for="skill in lead.skills"
								:key="skill.skill_id"
								class="badge badge-skill me-2"
							>
								{{ skill.skill_label }}
							</span>
						</td>
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
			leads: Array,
		}
	}
</script>