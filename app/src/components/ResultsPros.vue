<template>
	<div class="p-3 rounded-2 content-box bg-dark text-light">
		<VModal :modalTitle="'Match'" :modalID="'match'">
			<MatchContent :chosenMatch="currentMatch"/>
		</VModal>
		<h2 v-if="project.name">Pro search results for {{ project.name }}</h2>
		<transition name="fadeHeight">
			<table v-if="project.matches" class="table table-dark table-striped text-light">
				<thead>
					<tr>
						<th scope="col">#</th>
						<th scope="col">Name</th>
						<th scope="col">Available?</th>
					</tr>
				</thead>
				<tbody>
					<tr v-for="(user, index) in project.matches" :key="user.uid" :class="`tier tier--${user.tier}`">
						<th scope="row"><span :class="`tier__ball tier__ball--${user.tier}`">{{ index + 1 }}</span></th>
						<td><a href="#"
							data-bs-toggle="modal"
							data-bs-target="#hulaModalmatch"
							v-on:click="currentMatch = user
						">{{ user.first_name }} {{ user.last_name }}</a></td>
						<td>{{ user.is_available }}</td>
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
				matches: {},
				show: false,
			}
		},
		props: {
			project: {}
		},
		components: {
			VModal,
			MatchContent
		},
	}
</script>