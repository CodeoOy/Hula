<template>
	<div class="container mt-4">
		<VModal :modalTitle="form_title" :modalID="'admin'">
			<component :is='modalComponent' :chosenProject="this.$store.state.chosenproject"/>
		</VModal>
		<div class="row gx-4">
			<div class="col-md-4">
				<div class="p-3 mb-4 rounded-2 content-box bg-dark text-light">
					<h1>Welcome!</h1>
					<p><a href="#" data-bs-toggle="modal" data-bs-target="#hulaModaladmin" v-on:click.native.prevent="normalizeChosenProject">Add project</a></p>
					<p><a href="#" data-bs-toggle="modal" data-bs-target="#hulaModaladmin" v-on:click="form_title = 'New skill', chosenForm = 'skill'">Add skill</a></p>
					<p><a href="#" data-bs-toggle="modal" data-bs-target="#hulaModaladmin" v-on:click="form_title = 'New scope', chosenForm = 'scope'">Add scope</a></p>
					<p><a href="#" data-bs-toggle="modal" data-bs-target="#hulaModaladmin" v-on:click="form_title = 'New category', chosenForm = 'category'">Add category</a></p>
					<p><a href="#" data-bs-toggle="modal" data-bs-target="#hulaModaladmin" v-on:click="form_title = 'New level', chosenForm = 'scopelevel'">Add level</a></p>
					<p><a href="#" data-bs-toggle="modal" data-bs-target="#hulaModaladmin" v-on:click="form_title = 'Delete user', chosenForm = 'deleteuser'">Delete user</a></p>
				</div>
			</div>
			<div class="col-md-8">
				<ul class="nav nav-tabs nav-dark">
					<li class="nav-item">
						<a class="nav-link" v-bind:class="{ 'active': tab == 1, '': tab != 1 }" @click="tab = 1" href="#">Projects</a>
					</li>
					<li class="nav-item">
						<a class="nav-link" v-bind:class="{ 'active': tab == 2, '': tab != 2 }" @click="tab = 2" href="#">Users</a>
					</li>
					<li class="nav-item">
						<a class="nav-link" v-bind:class="{ 'active': tab == 3, '': tab != 3 }" @click="tab = 3" href="#">Skills</a>
					</li>
				</ul>
				<div class="p-3 mb-4 rounded-2 content-box bg-dark text-light">
					<!--<AdminListProjects v-if="tab == 1" />-->
					<AdminListProjects @project-chosen="updateProject" v-if="tab == 1" />
					<AdminListUsers v-if="tab == 2"  />
					<AdminListSkills v-if="tab == 3"  />
				</div>
			</div>
		</div>
	</div>
</template>

<script>
	import VModal from '../components/VModal.vue'
	import FormProject from '../forms/FormProject.vue'
	import FormSkill from '../forms/FormSkill.vue'
	import FormSkillScope from '../forms/FormSkillScope.vue'
	import FormSkillCategory from '../forms/FormSkillCategory.vue'
	import FormSkillScopeLevel from '../forms/FormSkillScopeLevel.vue'
	import FormUserDelete from '../forms/FormUserDelete.vue'
	import AdminListProjects from '../components/AdminListProjects.vue'
	import AdminListSkills from '../components/AdminListSkills.vue'
	import AdminListUsers from '../components/AdminListUsers.vue'
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
			VModal,
			FormProject,
			FormSkill,
			FormSkillScope,
			FormSkillCategory,
			FormSkillScopeLevel,
			FormUserDelete,
			AdminListProjects,
			AdminListSkills,
			AdminListUsers,
		},
		methods: {
			updateProject(value) {
				console.log("update project emit fired")
				this.chosenForm = 'project'
				//this.chosenProject = this.$store.state.chosenproject
				this.form_title = value
			},
			normalizeChosenProject() {
				this.form_title = 'New Project'
				this.chosenForm = 'project'
				this.$store.commit('resetChosenProject')
			}
		},
		computed: {
			modalComponent() {
				const components = {
					project: FormProject,
					skill: FormSkill,
					scope: FormSkillScope,
					category: FormSkillCategory,
					scopelevel: FormSkillScopeLevel,
					deleteuser: FormUserDelete,
				}
				return components[this.chosenForm]
			}
		}
	}
</script>