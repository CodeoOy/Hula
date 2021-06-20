<template>
	<div>
		<VModal :modalTitle="'Match'" :modalID="'match'">
			<MatchContent :match="currentMatch"/>
		</VModal>
		<div class="modal fade" v-bind:class="{ 'show db': show, '': !show }">
			<div class="modal-dialog">
				<div class="modal-content p-3 rounded-2 content-box bg-dark text-light">
					<h2>Match</h2>
					<table class="table table-dark table-striped text-light">
						<thead>
							<tr>
								<th></th>
								<th scope="col">{{ currentMatch.projectname }}</th>
								<th scope="col">{{ currentMatch.firstname }} {{ currentMatch.lastname }}</th>
							</tr>
						</thead>
						<tbody>
							<tr>
								<td>{{ currentMatch.skillname }}</td>
								<td>{{ currentMatch.required_minyears }}</td>
								<td>{{ currentMatch.user_years }}</td>
							</tr>
						</tbody>
					</table>
					<button class="btn btn-gradient" v-on:click="show = false">Close</button>
				</div>
			</div>
		</div>
		<ul class="matches">
			<li class="match" v-for="match in matches" :key="match.projectname">
				<a href="#" data-bs-toggle="modal" data-bs-target="#hulaModalmatch" v-on:click="currentMatch = match">
					<div class="match__bg"></div>
					<div class="match__pro">
						<img :src="'/public/assets/' + match.firstname + '.jpg'">
						<h4>{{ match.firstname }}</h4>
					</div>
					<div class="match__project">
						<img :src="'/public/assets/' + match.projectname + '.jpg'">
						<h4>{{ match.projectname }}</h4>
					</div>
				</a>
			</li>
		</ul>
	</div>
</template>

<script>
	import VModal from '../components/VModal.vue'
	import MatchContent from '../components/MatchContent.vue'
	export default {
		name: 'FeatMatches',
		data() {
			return {
				currentMatch: {},
				matches: {},
				show: false,
			}
		},
		components: {
			VModal,
			MatchContent
		},
		methods: {
			getMatches() { 
				fetch('api/matches', {
					method: 'GET',
					headers: {"Content-Type": "application/json"}
				})
				.then((response) => response.json())
				.then(response => {
					this.matches = response.slice(0,4);
				})    
				.catch((errors) => {
					console.log("Matches not found. Error data: ");
					console.log(errors)
				})
			}
		},
		mounted() {
			this.getMatches()
		}
	}
</script>