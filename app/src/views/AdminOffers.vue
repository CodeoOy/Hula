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
			}
		},
		components: {
			FormConfirmAction,
			VAutoComplete,
		},
		methods: {
			getProjectName(id) {
				let project = this.$store.state.projects.find(project => project.id == id)
				if (project) {
					return project.name
				} else {
					return 'Unknown'
				}
			},
			getUserName(id) {
				let user = this.users.find(user => user.id == id)
				if (user) {
					return user.name
				} else {
					return 'Unknown'
				}
			},
			autoCompleteAction(value) {
				this.filteredOffers = value
			},
			getAllUsers() {
				fetch('/api/users', {method: 'GET'})
				.then(response => { 
					return (response.status >= 200 && response.status <= 299) ? response.json() : this.$store.commit('errorHandling', response)
				})
				.then(response => { 
					this.users = response;
				})
			}
		},
		computed: {
			offers() {
				fetch('/api/offers', {method: 'GET'})
				.then(response => { 
					return (response.status >= 200 && response.status <= 299) ? response.json() : this.$store.commit('errorHandling', response)
				})
				.then(response => { 
					let offers = response;
					offers.forEach(offer => {
						offer.project_name = this.getProjectName(offer.project_id)
						offer.user_name = this.getUserName(offer.user_id)
					})
					console.log("Fetched offers")
					console.log(offers)
					return offers
				})
			}
		},
		mounted() {
			this.getAllUsers()
		},
	}
</script>
