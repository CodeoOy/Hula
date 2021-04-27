<template>
	<ul class="matches">
		<li class="match" v-for="match in matches" :key="match.projectname">
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
</template>

<script>
	export default {
		name: 'FeatMatches',
		data() {
			return {
				matches: {},
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
					this.matches = response;
				})    
			}
		},
		mounted() {
			this.getMatches()
		}
	}
</script>