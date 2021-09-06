<template>
	<div>
		<div class='d-sm-flex flex-row justify-content-between align-items-start'>
			<h2 class='h2'>Offers</h2>
			<div>
				<VAutoComplete
					:suggestions='offers'
					:dropdown='false'
					filterProperties='project_name'
					placeholder='Filter offers'
					v-on:auto-complete='onAutoComplete'
				></VAutoComplete>
			</div>
		</div>
		<div class='table-responsive'>
			<table class='table table-dark table-striped text-light'>
				<thead>
					<tr>
						<th scope='col'>Project name</th>
						<th scope='col'>User name</th>
						<th scope='col'>Sold?</th>
						<th scope='col'>Comments</th>
						<th scope='col'>Actions</th>
					</tr>
				</thead>
				<tbody>
					<tr v-for='offer in filteredOffers' :key='offer.id'>
						<td>{{ offer.project_name }}</td>
						<td>{{ offer.user_name }}</td>
						<td>{{ offer.sold }}</td>
						<td>{{ offer.comments }}</td>
						<td>
							<button class='btn btn-unstyled' v-on:click='edit(offer)'><i class='bi-pencil-fill me-2'></i></button>
							<button class='btn btn-unstyled' v-on:click='confirmDelete(offer)'><i class='bi-trash-fill me-2'></i></button>
						</td>
					</tr>
				</tbody>
			</table>
		</div>
	</div>
</template>

<script>
	import FormOffer from '../forms/FormOffer.vue'
	import VAutoComplete from '../components/VAutoComplete.vue'

	export default {
		name: 'AdminListOffers',

		components: {
			VAutoComplete,
		},

		data () {
			return {
				filteredOffers: [],
				users: [],
				offers: [],
			}
		},

		async mounted() {
			const [
				_,
				users,
				offers,
			] = await Promise.all([
				this.$store.dispatch('getProjects'),
				this.$api.users.get(),
				this.$api.offers.get(),
			])
			this.users = users
			this.offers = offers

			this.addUserNames()
			this.addProjectNames()
		},

		methods: {
			addUserNames() {
				this.offers.forEach(offer => {
					const user = this.users.find(({ id }) => id == offer.user_id)
					offer.user_name = user ? `${user.firstname} ${user.lastname}` : 'Unknown'
				})
			},

			addProjectNames() {
				this.offers.forEach(offer => {
					const project = this.$store.state.projects.find(project => project.id == offer.project_id)
					offer.project_name = project ? project.name : 'Unknown'
				})
			},

			async loadData() {
				this.offers = await this.$api.offers.get()
				this.addUserNames()
				this.addProjectNames()
			},

			onAutoComplete(value) {
				this.filteredOffers = value
			},

			async confirmDelete(offer) {
				const success = await this.$confirm.delete('offer', offer)
				if (success) this.loadData()
			},

			async edit(offer) {
				const result = await this.$modal({ title: 'Edit offer', component: FormOffer, props: offer })
				if (result) this.loadData()
			}
		},
	}
</script>
