<template>
	<div v-if="users.length">
		<div class="table-responsive">
			<table class="table table-striped" :class='$colorScheme.table'>
				<thead>
					<tr>
						<th scope="col"></th>
						<th scope="col">Name</th>
						<th scope="col" class='text-center'>Mandatory skills</th>
						<th scope="col">Matched skills</th>
						<th scope="col" class='text-center'>Tier</th>
						<th scope="col" class='text-center'>Available</th>
					</tr>
				</thead>
				<tbody>
					<tr v-for="(user, index) in users" :key="user.user_id" :class="`tier tier-${user.tier}`">
						<th scope="row"><div class="tier-circle shadow-sm" :style='{ zIndex: users.length - index }'>{{ index + 1 }}</div></th>
						<td>
							<button v-on:click='showMatch(user)' class='btn btn-unstyled'>
								{{ user.user_first_name }} {{ user.user_last_name }} 
							</button>
							<i v-if='user.user_favorite' class='bi-star-fill text-yellow ms-2 float-end'></i>
						</td>
						<td class='text-center'>{{ user.hasMandatory }}</td>
						<td><span class="badge badge-skill me-2" v-for="skill in user.skills" :key="skill.skill_id">{{ skill.skill_label }}</span></td>
						<td class='text-center'>{{ user.tier }}</td>
						<td class='text-center'>{{ user.isAvailable }}</td>
					</tr>
				</tbody>
			</table>
		</div>
	</div>
</template>

<script>
	import VModal from '../components/VModal.vue'
	import MatchContent from '../components/MatchContent.vue'
	export default {
		name: 'ResultsPros',
		data() {
			return {
				currentMatch: {},
			}
		},
		props: {
			project: {
				type: Object,
				required: true,
			},
			matches: {
				type: Object,
				required: true,
			},
		},
		components: {
			VModal,
			MatchContent
		},
		computed: {
			users() {
				if (this.matches) {
					const users = Object.values(this.matches).map(user => {
						user.hasMandatory = this.hasAllMandatorySkills(user)
						user.tier = this.tier(user)
						user.isAvailable = this.isAvailable(user)
						return user
					})
					return users.sort((a, b) => (a.tier > b.tier) ? 1 : (a.tier === b.tier) ? ((a.isAvailable > b.isAvailable) ? 1 : -1) : -1 )
				}
			},
			mandatorySkills() {
				const skills = this.project.skills
					? this.project.skills
					: this.project.needs.reduce((skills, need) => [
						...skills,
						...need.skills.map(skill => ({
							skill_label: skill.skill_label,
							skill_mandatory: skill.mandatory,
						}))
					], [])

				return skills.filter(skill => skill.skill_mandatory)
			},
		},
		methods: {
			tier(user) {
				let hasMandatory = this.hasAllMandatorySkills(user)
				let isAvailable = this.isAvailable(user)
				if (hasMandatory == true && isAvailable == true) {
					return 1
				} else if (hasMandatory == true && isAvailable == false) {
					return 2
				} else if (hasMandatory == false && isAvailable == true) {
					return 3
				} else {
					return 4
				}
			},
			hasAllMandatorySkills(user) {
				if (this.mandatorySkills.length) {
					return this.mandatorySkills.every(skill => {
						return user.skills.some(match => {
							return match.skill_label === skill.skill_label
							&& match.user_index >= match.required_index
							&& match.user_years >= match.required_minyears
							&& (match.user_years <= match.required_maxyears || match.required_maxyears === null)
						})
					})
				} else {
					return false
				}
			},
			isAvailable(user) {
				return user.skills.every(match => {
					return match.required_load >= match.user_load
				})
			},
			showMatch(props) {
				this.$modal({
					title: 'Match',
					component: MatchContent,
					props,
				})
			},
		},
	}
</script>