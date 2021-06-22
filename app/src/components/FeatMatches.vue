<template>
	<div>
		<VModal :modalTitle="'Match'" :modalID="'match'">
			<MatchContent :match="currentMatch"/>
		</VModal>
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