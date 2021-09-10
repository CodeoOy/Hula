<template>
	<div class='card shadow' :class='$colorScheme.card'>
		<div class='card-header'>
			<div class="d-sm-flex flex-row justify-content-between align-items-center">
				<h1 class="h3 mb-0">Users</h1>
				<div class='d-flex'>
					<div class='input-group me-2'>
						<VAutoComplete
							v-if="initialUsers.length" 
							:suggestions="initialUsers" 
							:placeholder="'filter users'"
							:dropdown="false"
							:filterProperties="['firstname', 'lastname']"
							v-on:auto-complete="autoCompleteAction"
						/>
						<button class='btn btn-secondary dropdown-toggle' type='button' id='filtersDropdown' data-bs-toggle='dropdown' data-bs-auto-close='outside' aria-expanded='false'>
							<i aria-label='Filters' class='bi bi-gear-fill'></i>
						</button>
						<ul class='dropdown-menu dropdown-menu-end' aria-labelledby='filtersDropdown'>
							<li class='px-2'>
								<div class='form-check'>
									<label for='hidden'>Don't show hidden</label>
									<input v-model='filters.hideHidden' type='checkbox' class='form-check-input' id='hidden' />
								</div>
							</li>
							<li class='px-2'>
								<div class='form-check'>
									<label for='employees'>Employees only</label>
									<input v-model='filters.employeesOnly' type='checkbox' class='form-check-input' id='employees' />
								</div>
							</li>
						</ul>
					</div>
					<button class="btn btn-primary gradient flex-shrink-0" v-on:click="inviteUser()">Invite a user</button>
				</div>
			</div>
		</div>
		<div class='card-body'>
			<div class="table-responsive">
				<table class="table table-striped" :class='$colorScheme.table'>
					<thead>
						<tr>
							<th scope="col">User</th>
							<th scope="col">Email</th>
							<th scope="col" class='text-end'>Actions</th>
						</tr>
					</thead>
					<tbody>
						<tr v-for="user in filteredUsers" :key="user.id">
							<td>
								<span>
									<router-link :to="{ name: 'user', params: { id: user.id}}">
										<VAvatar :user_id="user.id" :firstname="user.firstname" :lastname="user.lastname" />
										{{ user.firstname }} {{ user.lastname }}
									</router-link>
									<i v-if='user.is_hidden' class="bi-eye-slash-fill ms-2 float-end"></i>
								</span>
							</td>
							<td>{{ user.email }}</td>
							<td class='text-end'>
								<button class='btn btn-unstyled' v-on:click="confirmDelete(user)"><i class="bi-trash-fill me-2"></i></button>
							</td>
						</tr>
					</tbody>
				</table>
			</div>
		</div>
	</div>
</template>

<script>
	import VAvatar from '../components/VAvatar.vue'
	import VModal from '../components/VModal.vue'
	import VAutoComplete from '../components/VAutoComplete.vue'

	export default {
		name: 'AdminListUsers',
		data() {
			return {
				initialUsers: [],
				autoCompletedUsers: [],
				filters: {
					hideHidden: false,
					employeesOnly: false,
				},
			}
		},
		components: {
			VModal,
			VAvatar,
			VAutoComplete,
		},
		computed: {
			filteredUsers() {
				return this.autoCompletedUsers
					.filter(user => user.is_hidden ? !this.filters.hideHidden : true)
					.filter(user => this.filters.employeesOnly ? user.is_employee : true)
			},
		},
		methods: {
			async getUsers() {
				this.initialUsers = await this.$api.users.get()
			},
			autoCompleteAction(value) {
				this.autoCompletedUsers = value
			},
			async inviteUser() {
				// TODO: Implement this
			},
			async confirmDelete(user) {
				const success = await this.$confirm.delete('user', user)
				if (success) this.getUsers()
			},
		},
		async mounted() {
			this.getUsers()
		}
	}
</script>
