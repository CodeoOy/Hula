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
					<td>
						<VSkillBadge
							v-for="skill in match.skills"
							:key="skill.skill_id"
							:label='skill.skill_label'
							:mandatory='skill.skill_mandatory'
							:percentage='skill.skill_percentage'
							class='me-2'
						/>
					</td>
				</tr>
			</tbody>
		</table>
	</div>
</template>

<script>
	import VSkillBadge from './VSkillBadge.vue'

	export default {
		name: 'VMatchesForUser',

		components: {
			VSkillBadge,
		},

		props: {
			matches: Array,
		},
	}
</script>
