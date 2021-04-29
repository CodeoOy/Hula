<template>
	<div>
		<div class="modal fade" v-bind:class="{ 'show db': show, '': !show }">
			<div class="modal-dialog">
				<div class="modal-content p-3 rounded-2 content-box bg-dark text-light">
					<h2>Match</h2>
					<table class="table table-dark table-striped text-light">
						<thead>
							<tr>
								<th></th>
								<th scope="col">{{ currentmatch.projectname }}</th>
								<th scope="col">{{ currentmatch.firstname }} {{ currentmatch.lastname }}</th>
							</tr>
						</thead>
						<tbody>
							<tr>
								<td>{{ currentmatch.skillname }}</td>
								<td>{{ currentmatch.required_minyears }}</td>
								<td>{{ currentmatch.user_years }}</td>
							</tr>
						</tbody>
					</table>
					<button class="btn btn-primary" v-on:click="show = false">Close</button>
				</div>
			</div>
		</div>
		<ul class="matches">
			<li v-on:click="show = true, currentmatch = match" class="match" v-for="match in matches" :key="match.projectname">
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
				currentmatch: {},
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
					this.matches = response.slice(0,4);
				})    
			}
		},
		mounted() {
			this.getMatches()
		}
	}
</script>