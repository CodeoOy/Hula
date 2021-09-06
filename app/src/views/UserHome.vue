<template>
	<div class="container mt-4">
		<div class="p-3 mb-4 rounded-2 content-box bg-dark text-light">
			<h1 class="h2">Projects matching your skills</h1>
			<VMatchesForUser :matches='matches' />
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