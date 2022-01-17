<template>
	<div class="container">
		<div class="card shadow" :class='$colorScheme.card'>
			<div class='card-header'>
				<h1 class="h3 mb-0">Projects matching your skills</h1>
				<div class="alert alert-light" role="alert">
					<small><i class="bi-info-circle-fill" title='Delay info'></i> It may take up to a minute until the projects are updated.</small>
				</div>
				<button class='btn btn-primary gradient float-start' v-on:click='addSkill()'>Add skills</button>
			</div>
			<div class='card-body'>
				<VMatchesForUser v-if='matches.length' :user='user' :matches='matches' />
				<div v-else class='fs-3 fw-light text-muted text-center p-4'>No matches</div>
			</div>
		</div>
	</div>
</template>

<script>
	import VMatchesForUser from '@components/VMatchesForUser.vue'

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

		methods: {
				addSkill() {
					this.$router.push({name: 'user', params: {id: this.user.id}})
			}
		}
	}
</script>
