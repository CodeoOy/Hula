<template>
	<ul>
		<li v-for="match in matches" :key="match.id">
			{{ match }}
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