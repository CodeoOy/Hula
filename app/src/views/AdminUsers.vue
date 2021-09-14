<template>
	<div class='card shadow' :class='$colorScheme.card'>
		<div class='card-header'>
			<div class="d-flex justify-content-between align-items-center flex-wrap">
				<h1 class="h3 flex-grow-1 mb-0">Users</h1>
				<div class='input-group order-last mt-3 order-md-0 me-md-2 w-md-auto mt-md-0'>
					<VAutoComplete
						v-if="initialUsers.length" 
						:suggestions="initialUsers" 
						:placeholder="'filter users'"
						:dropdown="false"
						:filterProperties="['firstname', 'lastname', 'autoCompleteSkills']"
						v-on:auto-complete="autoCompleteAction"
					/>
					<button class='btn btn-secondary' type='button' data-bs-toggle="collapse" data-bs-target="#filters" aria-expanded="false" aria-controls="filters">
						<i aria-label='Filters' class='bi bi-gear-fill'></i>
					</button>
				</div>
				<button class="btn btn-primary gradient flex-shrink-0" v-on:click="inviteUser()">Invite a user</button>
			</div>
			<div class="collapse float-end" id="filters">
				<div class='mt-3 mb-2'>
					<span class='fw-bold me-3'>Include:</span>
					<div class='form-check form-check-inline'>
						<label for='hidden'>hidden</label>
						<input v-model='filters.hidden' type='checkbox' class='form-check-input' id='hidden' />
					</div>
					<div class='form-check form-check-inline'>
						<label for='employees'>employees only</label>
						<input v-model='filters.employees' type='checkbox' class='form-check-input' id='employees' />
					</div>
				</div>
			</div>
		</div>
		<div class='card-body'>
			<div v-if='filteredUsers.length'>
				<table class="table table-striped mb-0 table-stack-mobile" :class='$colorScheme.table'>
					<thead>
						<tr>
							<th scope="col">User</th>
							<th scope="col">Email</th>
							<th scope="col">Skills</th>
							<th scope="col" class='text-end'>Actions</th>
						</tr>
					</thead>
					<tbody>
						<tr v-for="user in filteredUsers" :key="user.id" class='context'>
							<td data-label='User'><div class='table-stack-mobile-cell'>
								<span>
									<router-link :to="{ name: 'user', params: { id: user.id}}">
										<VAvatar :user_id="user.id" :firstname="user.firstname" :lastname="user.lastname" class='d-none d-md-inline-block' />
										{{ user.firstname }} {{ user.lastname }}
									</router-link>
									<i v-if='user.is_hidden' class="bi-eye-slash-fill ms-2 float-end"></i>
								</span>
							</div></td>
							<td data-label='Email'><div class='table-stack-mobile-cell' :set='email = user.email.split("@")'>{{ email[0] }}<wbr />@{{ email[1] }}</div></td>
							<td data-label='Skills'><div class='table-stack-mobile-cell'>
								<VSkillBadge
									v-for='skill in user.skills'
									:key='skill.id'
									:label='skill.skill_label'
									:percentage="skill.level_percentage"
									class='me-2'
								/>
							</div></td>
							<td class='text-end' data-label='Actions'><div class='table-stack-mobile-cell'>
								<div class='context-actions'>
									<button class='btn btn-unstyled' v-on:click="confirmDelete(user)"><i class="bi-trash-fill"></i></button>
								</div>
							</div></td>
						</tr>
					</tbody>
				</table>
			</div>
			<div v-else>
				<div class='fs-3 fw-light text-muted text-center p-4'>{{ noUsersMessage }}</div>
			</div>
		</div>
	</div>
</template>

<script>
	import VAvatar from '../components/VAvatar.vue'
	import VModal from '../components/VModal.vue'
	import VAutoComplete from '../components/VAutoComplete.vue'
import VSkillBadge from '../components/VSkillBadge.vue'

	export default {
		name: 'AdminListUsers',
		data() {
			return {
				initialUsers: [],
				autoCompletedUsers: [],
				filters: {
					hidden: true,
					employees: false,
				},
			}
		},
		components: {
			VModal,
			VAvatar,
			VAutoComplete,
VSkillBadge,
		},
		computed: {
			filteredUsers() {
				return this.autoCompletedUsers
					.filter(user => user.is_hidden ? this.filters.hidden : true)
					.filter(user => this.filters.employees ? user.is_employee : true)
			},
			noUsersMessage() {
				return this.initialUsers.length
					? 'No users matching the filter'
					: 'No users'
			}
		},
		methods: {
			async getUsers() {
				this.initialUsers = await this.$api.users.get()
				this.initialUsers.forEach(user => {
					user.autoCompleteSkills = user.skills.map(skill => skill.skill_label)
				})
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
		activated() {
			this.getUsers()
		}
	}
</script>
