<template>
	<div class='card shadow' :class='$colorScheme.card'>
		<div class='card-header'>
			<div class="d-flex justify-content-between align-items-center flex-wrap">
				<h1 class="h3 flex-grow-1 mb-0">Offers</h1>
				<VAutoComplete
					:suggestions='offers'
					:dropdown='false'
					:filterProperties='["project_name", "user_name", "comments"]'
					placeholder='Filter offers'
					v-on:auto-complete='onAutoComplete'
					class='mt-3 w-sm-auto mt-sm-0'
				/>
			</div>
		</div>
		<div class='card-body'>
			<div v-if='filteredOffers.length'>
				<table class='table table-striped table-stack-mobile mb-0' :class='$colorScheme.table'>
					<thead>
						<tr>
							<th scope='col'>Project name</th>
							<th scope='col'>User name</th>
							<th scope='col' class='text-center'>Sold</th>
							<th scope='col'>Comments</th>
							<th scope='col' class='text-end'>Actions</th>
						</tr>
					</thead>
					<tbody>
						<tr v-for='offer in filteredOffers' :key='offer.id' class='context'>
							<td data-label='Project'><div class='table-stack-mobile-cell'>{{ offer.project_name }}</div></td>
							<td data-label='User'><div class='table-stack-mobile-cell'>{{ offer.user_name }}</div></td>
							<td class='text-center' data-label='Sold'><div class='table-stack-mobile-cell'>{{ offer.sold }}</div></td>
							<td data-label='Comments'><div class='table-stack-mobile-cell'>{{ offer.comments }}</div></td>
							<td class='text-end' data-label='Actions'><div class='table-stack-mobile-cell'>
								<div class='context-actions'>
									<button class='btn btn-unstyled' v-on:click='edit(offer)'><i class='bi-pencil-fill me-2'></i></button>
									<button class='btn btn-unstyled' v-on:click='confirmDelete(offer)'><i class='bi-trash-fill'></i></button>
								</div>
							</div></td>
						</tr>
					</tbody>
				</table>
			</div>
			<div v-else>
				<div class='fs-3 fw-light text-muted text-center p-4'>{{ noOffersMessage }}</div>
			</div>
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

		computed: {
			noOffersMessage() {
				return this.offers.length
					? 'No offers matching the filter'
					: 'No offers'
			}
		},

		async activated() {
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
