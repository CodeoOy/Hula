<template>
	<div>
		<VModal :modalTitle="formTitle" :modalID="'Users'" v-on:modal-hidden="chosenForm = ''">
			<component 
				:is='modalComponent' 
				:chosenUser="chosenUser" 
				:url="url"
				:method="method"
				v-on:form-sent="hideModalUpdate"
			/>
		</VModal>
		<div class="d-sm-flex flex-row justify-content-between align-items-start">
			<h2 class="h2">Users</h2>
			<div>
				<VAutoComplete
					v-if="initialUsers.length" 
					:suggestions="initialUsers" 
					:selection="userName" 
					:placeholder="'filter users'"
					:dropdown="false"
					:filterProperties="'firstname'"
					v-on:auto-complete="autoCompleteAction"
				></VAutoComplete>
				<button
					class="btn btn-gradient"
					data-bs-toggle="modal"
					data-bs-target="#hulaModalUsers"
					v-on:click="formTitle = 'New User', chosenForm = 'CreateUser', chosenUser = {}, url='/api/users', method='POST'"
				>Invite a user</button>
			</div>
		</div>
		<transition name="fadeHeight">
			<div class="table-responsive">
				<table class="table table-dark table-striped text-light">
					<thead>
						<tr>
							<th scope="col">User</th>
							<th scope="col">Email</th>
							<th scope="col">Actions</th>
						</tr>
					</thead>
					<tbody>
						<tr v-for="user in filteredUsers" :key="user.id">
							<td>
								<span>
									<VAvatar :user_id="user.id" :firstname="user.firstname" :lastname="user.lastname" />
									<router-link :to="{ name: 'page-profile', params: { id: user.id}}">
										{{ user.firstname }} {{ user.lastname }}
									</router-link>
								</span>
							</td>
							<td>{{ user.email }}</td>
							<td class="hoverable-td">
								<a
									href="#"
									data-bs-toggle="modal"
									data-bs-target="#hulaModalUsers" 
									v-on:click="formTitle = `Delete ${user.firstname} ${user.lastname}?`, chosenForm = 'Delete', url = `/api/users/${user.id}`, method = 'DELETE'"
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
	import VAvatar from '../components/VAvatar.vue'
	import VModal from '../components/VModal.vue'
	import FormConfirmAction from '../forms/FormConfirmAction.vue'
	import VAutoComplete from '../components/VAutoComplete.vue'
	import { Modal } from 'bootstrap'
	export default {
		name: 'AdminListUsers',
		data() {
			return {
				formTitle: '',
				chosenForm: '',
				initialUsers: [],
				filteredUsers: [],
				chosenUser: {},
				userName: '',
				url: '',
				method: '',
			}
		},
		components: {
			VModal,
			VAvatar,
			FormConfirmAction,
			VAutoComplete,
		},
		methods: {
			hideModalUpdate() {
				this.getSkillCategories()
				this.getAllSkills()
				let modal = Modal.getInstance(document.querySelector('#hulaModalUsers'))
				modal.hide()
			},
			getAllUsers() {
				fetch('/api/users', {method: 'GET'})
				.then(response => { 
					return (response.status >= 200 && response.status <= 299) ? response.json() : this.$store.commit('errorHandling', response)
				})
				.then(response => { 
					this.initialUsers = response;
				})
				.catch((errors) => {
					this.$store.commit('errorHandling', errors)
				})
			},
			autoCompleteAction(value) {
				this.filteredUsers = value
			}
		},
		computed: {
			modalComponent() {
				const components = {
					Delete: FormConfirmAction,
				}
				return components[this.chosenForm]
			},
		},
		mounted() {
			this.getAllUsers()
		}
	}
</script>
