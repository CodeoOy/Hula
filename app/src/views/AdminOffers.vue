<template>
	<div>
		<div class="d-sm-flex flex-row justify-content-between align-items-start">
			<h2 class="h2">Offers</h2>
			{{ offers }}
			<div>
				<!-- <VAutoComplete 
					:suggestions="offers" 
					:selection="offerName" 
					:placeholder="'filter offers'"
					:dropdown="false"
					:filterProperties="'project_id'"
					v-on:auto-complete="autoCompleteAction"
				></VAutoComplete> -->
			</div>
		</div>
		<transition name="fadeHeight">
			<div class="table-responsive">
				<table class="table table-dark table-striped text-light">
					<thead>
						<tr>
							<th scope="col">Project name</th>
							<th scope="col">User name</th>
							<th scope="col">Sold?</th>
							<th scope="col">Actions</th>
						</tr>
					</thead>
					<tbody>
						<tr v-for="offer in offers" :key="offer.id">
							<td>{{ offer.project_name }}</td>
							<td>{{ offer.user_name }}</td>
							<td>{{ offer.sold }}</td>
							<td>
								<a
									href="#"
									data-bs-toggle="modal"
									data-bs-target="#hulaModalOffers" 
									v-on:click="formTitle = `Delete ${offer.id}?`, chosenForm = 'Delete', url = `/api/offers/${offer.id}`, method = 'DELETE'"
								><i class="bi-trash-fill me-2"></i></a>
							</td>
						</tr>
					</tbody>
				</table>
			</div>
		</transition>
	</div>
</template>

<script>
	import FormConfirmAction from '../forms/FormConfirmAction.vue'
	import VAutoComplete from '../components/VAutoComplete.vue'
	export default {
		name: 'AdminListOffers',
		data () {
			return {
				filteredOffers: [],
				offerName: '',
				users: [],
				offers: [],
			}
		},
		components: {
			FormConfirmAction,
			VAutoComplete,
		},
		methods: {
			addUserNames() {
				this.offers.forEach(offer => {
					const user = this.users.find(({ id }) => id == offer.user_id)
					offer.user_name = user ? `${user.firstname} ${user.firstname}` : 'Unknown'
				})
			},
			addProjectNames() {
				this.offers.forEach(offer => {
					const project = this.$store.state.projects.find(project => project.id == offer.project_id)
					offer.project_name = project ? project.name : 'Unknown'
				})
			},
			autoCompleteAction(value) {
				this.filteredOffers = value
			},
		},
		async mounted() {
			await Promise.all([
				this.$store.dispatch('getProjects'),
				this.users = await this.$api.users.get(),
				this.offers = await this.$api.offers.get(),
			])
			this.addUserNames()
			this.addProjectNames()
		},
	}
</script>
