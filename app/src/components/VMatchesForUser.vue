<template>
	<div class="table-responsive">
		<table class="table table-striped mb-0" :class='$colorScheme.table'>
			<thead>
				<tr>
					<th scope="col">Project</th>
					<th scope="col">Skills</th>
				</tr>
			</thead>
			<tbody>
				<tr v-for="match in matches" :key="match.id">
					<td><router-link :to='{ name: "project", params: { id: match.id } }'>{{ match.name }}</router-link></td>
					<td><span class="badge badge-skill me-2" v-for="skill in match.skills" :key="skill.skill_id">{{ skill.skill_label }}</span></td>
				</tr>
			</tbody>
		</table>
	</div>
</template>

<script>
	export default {
		name: 'VMatchesForUser',

		props: {
			matches: Array,
		},
	}
</script>
