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
					<td>
						<router-link :to='{ name: "project", params: { id: match.id } }'>{{ match.name }}</router-link>
						<i v-if='isFavorite(match)' class='bi-star-fill text-yellow ms-2 float-end'></i>
					</td>
					<td class='align-middle'>
						<div class='hstack gap-2 flex-wrap'>
							<VSkillBadge
								v-for="skill in match.skills"
								:key="skill.skill_id"
								:label='skill.skill_label'
								:mandatory='skill.skill_mandatory'
								:percentage='skill.skill_percentage'
							/>
						</div>
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
			user: {
				type: Object,
				required: true,
			},

			matches: {
				type: Array,
				required: true,
			},
		},

		methods: {
			isFavorite(project) {
				return project.matches.find(match => match.user_id == this.user.id && match.is_favorite)
			},
		},
	}
</script>
