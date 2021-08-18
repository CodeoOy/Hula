<template>
	<div class="p-3 rounded-2 content-box bg-dark text-light">
		<VModal :modalTitle="'Match'" :modalID="'match'">
			<MatchContent :chosenMatch="currentMatch"/>
		</VModal>
		<!-- <h2 v-if="project.name">Pro search results for {{ project.name }}</h2> -->
		<transition name="fadeHeight">
			<table v-if="matches" class="table table-dark table-striped text-light">
				<thead>
					<tr>
						<th scope="col"></th>
						<th scope="col">Name</th>
						<th scope="col">Available?</th>
					</tr>
				</thead>
				<tbody>
					<tr v-for="(user, index) in users" :key="user.user_first_name" :class="`tier tier--${tier(user)}`">
						<th scope="row"><span :class="`tier__ball tier__ball--${tier(user)}`" :style="`zIndex: ${index}`">{{ index + 1 }}</span></th>
						<td><a href="#"
							data-bs-toggle="modal"
							data-bs-target="#hulaModalmatch"
							v-on:click="currentMatch = user
						">{{ user.user_first_name }} {{ user.user_last_name }}</a></td>
						<td>{{ user.user_is_available }}</td>
						<!--<a href="#" v-on:click="getUserData(user.uid)">{{user.firstname}} {{ user.lastname }}</a>-->
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
			}
		},
		props: {
			project: {},
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
				return users
			},
		},
		methods: {
			tier(user) {
				return user.user_is_hidden ? 1 : 2
			}
		}
	}
</script>