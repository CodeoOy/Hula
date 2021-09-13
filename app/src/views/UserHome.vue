<template>
	<div class="container">
		<div class="card shadow" :class='$colorScheme.card'>
			<div class='card-header'>
				<h1 class="h3 mb-0">Projects matching your skills</h1>
			</div>
			<div class='card-body'>
				<VMatchesForUser v-if='matches.length' :matches='matches' />
				<div v-else class='fs-3 fw-light text-muted text-center p-4'>No matches</div>
			</div>
		</div>
	</div>
</template>

<script>
	import VMatchesForUser from '../components/VMatchesForUser.vue'

	export default {
		name: 'HomeUser',

		components: {
			VMatchesForUser,
		},

		computed: {
			user() {
				return this.$store.state.loggeduser
			},

			matches() {
				return this.$store.state.projects.filter(project => {
					return project.matches.some(match =>
						match.user_id == this.user.id
					)
				})			
			},
		},

		mounted() {
			if (!this.$store.state.projects.length) this.$store.dispatch('getProjects')
		},
	}
</script>