<template>
	<div>
		<h2 class="h2">Projects for {{ user.firstname }} {{ user.lastname }}</h2>
		<div class="table-responsive">
			<table v-if="projects.length" class="table table-dark table-striped text-light">
				<thead>
					<tr>
						<th scope="col">Project</th>
						<th scope="col">Skills</th>
					</tr>
				</thead>
				<tbody>
					<tr v-for="project in projects" :key="project.id">
						<td>
							<router-link :to="{ name: 'project', params: { id: project.id }}">
								{{ project.name }}
							</router-link>
							<i v-if='project.is_hidden' class="bi-eye-slash-fill ms-2 float-end"></i>
						</td>
						<td>
							<span
								v-for="skill in project.skills"
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
			user: Object,
			projects: Array,
		}
	}
</script>