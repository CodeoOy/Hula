<template>
	<div class="container mt-4">
		<ul class="nav nav-tabs nav-dark">
			<li class="nav-item">
				<a class="nav-link" v-bind:class="{ 'active': tab == 1, '': tab != 1 }" @click="tab = 1" href="#">Projects</a>
			</li>
			<li class="nav-item">
				<a class="nav-link" v-bind:class="{ 'active': tab == 2, '': tab != 2 }" @click="tab = 2" href="#">Users</a>
			</li>
			<li class="nav-item">
				<a class="nav-link" v-bind:class="{ 'active': tab == 3, '': tab != 3 }" @click="tab = 3" href="#">Skills & Categories</a>
			</li>
			<li class="nav-item">
				<a class="nav-link" v-bind:class="{ 'active': tab == 4, '': tab != 4 }" @click="tab = 4" href="#">Scopes & Levels</a>
			</li>
			<li class="nav-item">
				<a class="nav-link" v-bind:class="{ 'active': tab == 5, '': tab != 5 }" @click="tab = 5" href="#">Offers</a>
			</li>
		</ul>
		<div class="p-3 mb-4 rounded-2 content-box bg-dark text-light">
			<AdminListProjects @project-chosen="updateProject" v-if="tab == 1" />
			<AdminListUsers v-if="tab == 2" />
			<AdminListSkills v-if="tab == 3" />
			<AdminListScopes v-if="tab == 4" />
			<AdminListOffers v-if="tab == 5" />
		</div>
	</div>
</template>

<script>
	import AdminListProjects from '../components/AdminListProjects.vue'
	import AdminListSkills from '../components/AdminListSkills.vue'
	import AdminListUsers from '../components/AdminListUsers.vue'
	import AdminListScopes from '../components/AdminListScopes.vue'
	import AdminListOffers from '../components/AdminListOffers.vue'
	export default {
		name: 'Admin',
		data() {
			return {
				form_title: '',	
				tab: 1,
				chosenProject: {
					type: Object,
					default() {
						return { id: '' }
					}
				},
				chosenForm: '',
			}
		},
		components: {
			AdminListProjects,
			AdminListSkills,
			AdminListUsers,
			AdminListScopes,
			AdminListOffers,
		},
		methods: {
			updateProject(value) {
				this.chosenForm = 'project'
				//this.chosenProject = this.$store.state.chosenproject
				this.form_title = value
			},
			normalizeChosenProject() {
				this.form_title = 'New Project'
				this.chosenForm = 'project'
				this.$store.commit('resetChosenProject')
			}
		}
	}
</script>