<template>
	<div>
		<VModal :modalTitle="formTitle" modalID="Offers" v-on:modal-hidden="chosenForm = ''">
			<component 
				:is='modalComponent'
				:url="url"
				:method="method"
				v-on:form-sent="hideModalUpdate"
			/>
		</VModal>
		<div class="d-sm-flex flex-row justify-content-between align-items-start">
			<h2 class="h2">Offers</h2>
			<div v-if="offers">
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
			<div v-if="offers" class="table-responsive">
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
						<tr v-for="offer in filteredOffers" :key="offer.id">
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
								<a
									href="#"
									data-bs-toggle="modal"
									data-bs-target="#hulaModalOffers" 
									v-on:click="formTitle = `Edit offer`, chosenForm = 'Edit', url = `/api/offers/${offer.id}`, method = 'PUT'"
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
	import FormConfirmAction from '../forms/FormConfirmAction.vue'
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
			}
		},
		components: {
			FormOffer,
			FormConfirmAction,
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
		},
		computed: {
			modalComponent() {
				const components = {
					Delete: FormConfirmAction,
					Edit: FormOffer,
				}
				return components[this.chosenForm]
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
