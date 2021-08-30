<template>
	<div v-if="matches.length">
		<VModal :modalTitle="'Match'" :modalID="'match'" @modal-hidden='currentMatch = null'>
			<MatchContent v-if='currentMatch' :chosenMatch="currentMatch" :projectName="project.name"/>
		</VModal>
		<div class="table-responsive">
			<table v-if="matches" class="table table-dark table-striped text-light">
				<thead>
					<tr>
						<th scope="col"></th>
						<th scope="col">Name</th>
						<th @click="sort('hasMandatory')" scope="col">Has mandatory skills</th>
						<th scope="col">Matched skills</th>
						<th scope="col">Tier</th>
						<th scope="col">Available?</th>
					</tr>
				</thead>
				<tbody>
					<tr v-for="(user, index) in users" :key="user.user_id" :class="`tier tier--${user.tier}`">
						<th scope="row" class="tier__index"><span :class="`tier__ball tier__ball--${user.tier}`" :style="`zIndex: ${index}`">{{ index + 1 }}</span></th>
						<td><a href="#"
							data-bs-toggle="modal"
							data-bs-target="#hulaModalmatch"
							v-on:click="currentMatch = user
						">{{ user.user_first_name }} {{ user.user_last_name }}</a></td>
						<td>{{ user.hasMandatory }}</td>
						<td><span class="badge" v-for="match in user.matches" :key="match.skill_id">{{ match.skill_label }}</span></td>
						<td>{{ user.tier }}</td>
						<td>{{ user.isAvailable }}</td>
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
				type: Array,
				required: true,
			},
		},
		components: {
			VModal,
			MatchContent
		},
		computed: {
			users() {
				if (this.matches.length) {
					const users = Object.values(this.matches.reduce((users, match) => {
						const {
							user_id,
							user_first_name,
							user_last_name,
							user_is_hidden,
							project_id,
							...rest
						} = match
						if (user_id in users) {
							users[user_id].matches.push(rest)
						} else {
							users[user_id] = {
								user_id,
								user_first_name,
								user_last_name,
								user_is_hidden,
								project_id,
								matches: [rest],
							}
						}
						users[user_id].hasMandatory = this.hasAllMandatorySkills(users[user_id])
						users[user_id].tier = this.tier(users[user_id])
						users[user_id].isAvailable = this.isAvailable(users[user_id])
						return users
					}, {}))
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
						return user.matches.some(match => {
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
				return user.matches.every(match => {
					return match.required_load >= match.user_load
				})
			},
		},
	}
</script>