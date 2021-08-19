<template>
	<div class="p-3 rounded-2 content-box bg-dark text-light">
		<VModal :modalTitle="'Match'" :modalID="'match'">
			<MatchContent :chosenMatch="currentMatch"/>
		</VModal>
		{{ skills }}
		<!-- <h2 v-if="project.name">Pro search results for {{ project.name }}</h2> -->
		<transition name="fadeHeight">
			<table v-if="matches" class="table table-dark table-striped text-light">
				<thead>
					<tr>
						<th scope="col"></th>
						<th scope="col">Name</th>
						<th scope="col">Has mandatory skills</th>
						<th scope="col">Matched skills</th>
						<th scope="col">Available?</th>
					</tr>
				</thead>
				<tbody>
					<tr v-for="(user, index) in users" :key="user.user_first_name" :class="`tier tier--${tier(user)}`">
						<th scope="row" class="tier__index"><span :class="`tier__ball tier__ball--${tier(user)}`" :style="`zIndex: ${index}`">{{ index + 1 }}</span></th>
						<td><a href="#"
							data-bs-toggle="modal"
							data-bs-target="#hulaModalmatch"
							v-on:click="currentMatch = user
						">{{ user.user_first_name }} {{ user.user_last_name }}</a></td>
						<td>{{ hasAllMandatorySkills(user) }}</td>
						<td><span class="badge" v-for="match in user.matches" :key="match.skill_id">{{ match.skill_label }}</span></td>
						<td>{{ user.user_is_available }}</td>
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
					return users
				}, {}))
				if (users.length) {
					this.getProjectSkills(users[0].project_id)
				}
				return users
			},
		},
		methods: {
			tier(user) {
				return user.user_is_hidden ? 1 : 2
			},
			hasAllMandatorySkills(user) {
				return this.mandatorySkills.every(skill => {
					return user.matches.some(match => {
						return match.skill_label === skill.skill_label
					})
				})
			},
			hasAllSkills() {

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