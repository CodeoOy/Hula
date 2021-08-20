<template>
	<div class="p-3 rounded-2 content-box bg-dark text-light">
		<VModal :modalTitle="'Match'" :modalID="'match'">
			<MatchContent :chosenMatch="currentMatch"/>
		</VModal>
		<h2 v-if="project[0]">Pro search results for {{ project[0].name }}</h2>
		<p>Projects skills: <span class="me-2" v-for="skill in this.skills" :key="skill.skill_id">{{ skill.skill_label }}</span></p>
		<transition name="fadeHeight">
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
		</transition>
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
				show: false,
				project: {},
				skills: [],
				mandatorySkills: [],
			}
		},
		props: {
			matches: []
		},
		components: {
			VModal,
			MatchContent
		},
		computed: {
			users() {
				if (this.matches.length) {
					console.log("Running the users and using resources!")
					this.getProjectSkills(this.matches[0].project_id)
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
				return this.mandatorySkills.every(skill => {
					return user.matches.some(match => {
						return match.skill_label === skill.skill_label
						&& match.user_index >= match.required_index
						&& match.user_years >= match.required_minyears
						&& (match.user_years <= match.required_maxyears || match.required_maxyears === null)
					})
				})
			},
			isAvailable(user) {
				return user.matches.every(match => {
					return match.required_load >= match.user_load
				})
			},
			getProjectSkills(id) {
				this.project = this.$store.state.projects.filter(project => {
					return (project.id == id)
				})
				this.skills = this.project[0].skills
				this.mandatorySkills = this.skills.filter(skill => {
					return skill.skill_mandatory === true
				})
			}
		},
	}
</script>