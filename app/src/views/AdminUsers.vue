<template>
	<div class='card shadow' :class='$colorScheme.card'>
		<div class='card-header'>
			<div class="d-flex justify-content-between align-items-center flex-wrap">
				<h1 class="h3 flex-grow-1 mb-0">Users</h1>
				<div class='input-group order-last mt-3 order-md-0 me-md-2 w-md-auto mt-md-0'>
					<VFilter
						:items='users'
						:props='["firstname", "lastname", "skillLabels"]'
						placeholder='filter users'
						@filter='filterUsers'
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
				<table class="table table-striped mb-0 table-stack-mobile table-lg" :class='$colorScheme.table'>
					<thead>
						<tr>
							<th scope="col">User</th>
							<th scope="col">Skills</th>
							<th scope="col" class='text-end'>Actions</th>
						</tr>
					</thead>
					<transition-group name='flip-list' tag='tbody' @before-leave='onBeforeTrLeave'>
						<tr v-for="user in filteredUsers" :key="user.id" class='context'>
							<td data-label='User'><div class='table-stack-mobile-cell text-nowrap'>
								<div class='hstack gap-2'>
									<router-link :to="{ name: 'user', params: { id: user.id}}">
										<VAvatar
											:id="user.id"
											:firstName="user.firstname"
											:lastName="user.lastname"
											class='d-none d-md-inline-block me-2'
										/>
										<VHighlight :text='`${user.firstname} ${user.lastname}`' :keywords='keywords' />
									</router-link>
									<i v-if='user.is_hidden' class='bi-eye-slash-fill flex-grow-1 text-end'></i>
								</div>
								<div>{{ user.email }}</div>
							</div></td>
							<td data-label='Skills'><div class='table-stack-mobile-cell'>
								<div class='hstack gap-2 flex-wrap'>
									<VSkillBadge
										v-for='skill in user.skills'
										:key='skill.id'
										:label='skill.skill_label'
										:percentage="skill.level_percentage"
										:highlight='highlight(skill)'
									/>
								</div>
							</div></td>
							<td class='text-end' data-label='Actions'><div class='table-stack-mobile-cell'>
								<div class='context-actions'>
									<button class='btn btn-unstyled px-1 rounded' v-on:click="confirmDelete(user)"><i class="bi-trash-fill"></i></button>
								</div>
							</div></td>
						</tr>
					</transition-group>
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
	import VFilter from '../components/VFilter.vue'
	import VHighlight from '../components/VHighlight.vue'
	import VSkillBadge from '../components/VSkillBadge.vue'
	import FormRegister from '../forms/FormRegister.vue'
	import { onBeforeTrLeave } from '../transitions.js'

	export default {
		name: 'AdminListUsers',

		components: {
			VAvatar,
			VFilter,
			VHighlight,
			VSkillBadge,
		},

		data() {
			return {
				users: [],
				usersByKeyword: [],
				filters: {
					hidden: true,
					employees: false,
				},
				keywords: [],
			}
		},

		computed: {
			filteredUsers() {
				return this.usersByKeyword
					.filter(user => user.is_hidden ? this.filters.hidden : true)
					.filter(user => this.filters.employees ? user.is_employee : true)
			},

			noUsersMessage() {
				return this.users.length
					? 'No users matching the filter'
					: 'No users'
			},

			highlightPattern() {
				return new RegExp(this.keywords.join('|'))
			},
		},

		activated() {
			this.getUsers()
		},

		methods: {
			onBeforeTrLeave,

			async getUsers() {
				this.users = await this.$api.users.get()
				this.users.forEach(user => {
					user.skillLabels = user.skills.map(skill => skill.skill_label)
				})
			},

			filterUsers({ matches, keywords }) {
				this.usersByKeyword = matches
				this.keywords = keywords
			},

			highlight(skill) {
				return Boolean(this.keywords.length && skill.skill_label.toUpperCase().match(this.highlightPattern))
			},

			async inviteUser() {
				const result = await this.$modal({
					title: 'Invite a user',
					component: FormRegister,
				})

				if (result) this.$store.dispatch('getProjects')
			},

			async confirmDelete(user) {
				const success = await this.$confirm.delete('user', user)
				if (success) this.getUsers()
			},
		},
	}
</script>
