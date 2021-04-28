<template>
	<div>
		<div class="modal fade" v-bind:class="{ 'show db': show, '': !show }">
			<div class="modal-dialog">
				<div class="modal-content p-3 rounded-2 content-box bg-dark text-light">
					<h2>Match</h2>
					<button class="btn btn-primary" v-on:click="show = false">Close</button>
				</div>
			</div>
		</div>
		<ul class="matches">
			<li v-on:click="show = true" class="match" v-for="match in matches" :key="match.projectname">
				<div class="match__bg"></div>
				<div class="match__pro">
					<img :src="'/public/assets/' + match.firstname + '.jpg'">
					<h4>{{ match.firstname }}</h4>
				</div>
				<div class="match__project">
					<img :src="'/public/assets/' + match.projectname + '.jpg'">
					<h4>{{ match.projectname }}</h4>
				</div>
			</li>
		</ul>
	</div>
</template>

<script>
	export default {
		name: 'FeatMatches',
		data() {
			return {
				matches: {},
				show: false,
			}
		},
		methods: {
			getMatches: function() { 
				fetch('api/matches', {
					method: 'GET',
					headers: {"Content-Type": "application/json"}
				})
				.then((response) => response.json())
				.then(response => { 
					console.log(response);
					this.matches = response.slice(0,4);
				})    
			}
		},
		mounted() {
			this.getMatches()
		}
	}
</script>