<template>
	<div>
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
								v-on:click.prevent="confirmDelete(user)"
							><i class="bi-trash-fill me-2"></i></a>
						</td>
					</tr>
				</tbody>
			</table>
		</div>
	</div>
</template>

<script>
	import VAvatar from '../components/VAvatar.vue'
	import VModal from '../components/VModal.vue'
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
			VAutoComplete,
		},
		methods: {
			async hideModalUpdate() {
				this.initialUsers = await this.$api.users.get()
				let modal = Modal.getInstance(document.querySelector('#hulaModalUsers'))
				modal.hide()
			},
			autoCompleteAction(value) {
				this.filteredUsers = value
			},
			async confirmDelete(user) {
				const success = await this.$confirm.delete('user', user)
				if (success) this.initialUsers = await this.$api.users.get()
			},
		},
		async mounted() {
			this.initialUsers = await this.$api.users.get()
		}
	}
</script>
