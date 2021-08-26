<template>
	<div>
		<VModal :modalTitle="formTitle" modalID="Offers" v-on:modal-hidden="chosenForm = ''">
			<component 
				:is='modalComponent'
				:url="url"
				:chosenOfferID="chosenOfferID"
				:method="method"
				v-on:form-sent="hideModalUpdate"
			/>
		</VModal>
		<div class="d-sm-flex flex-row justify-content-between align-items-start">
			<h2 class="h2">Offers</h2>
			<div>
				<VAutoComplete 
					:suggestions="offers" 
					:selection="offerName" 
					:placeholder="'filter offers'"
					:dropdown="false"
					:filterProperties="'project_name'"
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
							<th scope="col">Comments</th>
							<th scope="col">Actions</th>
						</tr>
					</thead>
					<tbody>
						<tr v-for="offer in filteredOffers" :key="offer.id">
							<td>{{ offer.project_name }}</td>
							<td>{{ offer.user_name }}</td>
							<td>{{ offer.sold }}</td>
							<td>{{ offer.comments }}</td>
							<td>
								<a
									href="#"
									v-on:click.prevent="confirmDelete(offer)"
								><i class="bi-trash-fill me-2"></i></a>
								<a
									href="#"
									data-bs-toggle="modal"
									data-bs-target="#hulaModalOffers" 
									v-on:click="formTitle = `Edit offer`, chosenOfferID = offer.id, chosenForm = 'Edit', url = `/api/offers/${offer.id}`, method = 'PUT'"
								><i class="bi-pencil-fill me-2"></i></a>
							</td>
						</tr>
					</tbody>
				</table>
			</div>
		</transition>
	</div>
</template>

<script>
	import FormOffer from '../forms/FormOffer.vue'
	import VAutoComplete from '../components/VAutoComplete.vue'
	import VModal from '../components/VModal.vue'
	import { Modal } from 'bootstrap'
	export default {
		name: 'AdminListOffers',
		data () {
			return {
				filteredOffers: [],
				offerName: '',
				users: [],
				offers: [],
				url: '',
				method: '',
				formTitle: '',
				chosenForm: '',
				chosenOfferID: '',
			}
		},
		components: {
			FormOffer,
			VAutoComplete,
			VModal,
		},
		methods: {
			async hideModalUpdate() {
				this.offers = await this.$api.offers.get()
				this.addUserNames()
				this.addProjectNames()
				let modal = Modal.getInstance(document.querySelector('#hulaModalOffers'))
				modal.hide()
			},
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
			autoCompleteAction(value) {
				this.filteredOffers = value
			},
			async confirmDelete(offer) {
				const success = await this.$confirm.delete('offer', offer)
				if (success) {
					this.offers = await this.$api.offers.get()
					this.addUserNames()
					this.addProjectNames()
				}
			},
		},
		computed: {
			modalComponent() {
				const components = {
					Edit: FormOffer,
				}
				return components[this.chosenForm]
			},
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
	}
</script>
