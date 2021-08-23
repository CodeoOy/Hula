<template>
	<div>
		<div class="d-flex flex-row justify-content-between align-items-start">
			<h2 class="h2">Offers</h2>
			<div>
				<VAutoComplete
					v-if="offers.length" 
					:suggestions="offers" 
					:selection="offerName" 
					:placeholder="'filter offers'"
					:dropdown="false"
					:filterProperties="'project_id'"
					v-on:auto-complete="autoCompleteAction"
				></VAutoComplete>
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
							<td>{{ offer.project_id }}</td>
							<td>{{ offer.user_id }}</td>
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
				offers: [],
				offerName: '',
			}
		},
		components: {
			FormConfirmAction,
			VAutoComplete,
		},
		methods: {
			getAllOffers () {
				fetch('/api/offers', {method: 'GET'})
				.then(response => { 
					return (response.status >= 200 && response.status <= 299) ? response.json() : this.$store.commit('errorHandling', response)
				})
				.then(response => { 
					this.offers = response;
				})    
				.catch((errors) => {
					this.$store.commit('errorHandling', errors)
				})
			},
			autoCompleteAction(value) {
				this.filteredOffers = value
			}
		},
		mounted() {
			this.getAllOffers()
		},
	}
</script>
