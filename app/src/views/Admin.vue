<template>
	<div class="container mt-4">
		<VModal :modal_title="form_title">
			<component :is='modalComponent' :chosenproject="chosenproject"/>
		</VModal>
		<div class="row gx-4">
			<div class="col-md-4">
				<div class="p-3 mb-4 rounded-2 content-box bg-dark text-light">
					<h1>Welcome {{ this.$store.state.loggeduser.firstname }}!</h1>
					<p><a href="#" data-bs-toggle="modal" data-bs-target="#hulaModal" v-on:click="chosenform = 'project'">Add project</a></p>
					<p><a href="#" data-bs-toggle="modal" data-bs-target="#hulaModal" v-on:click="chosenform = 'skill'">Add skill</a></p>
					<p><a href="#" data-bs-toggle="modal" data-bs-target="#hulaModal" v-on:click="chosenform = 'scope'">Add scope</a></p>
					<p><a href="#" data-bs-toggle="modal" data-bs-target="#hulaModal" v-on:click="chosenform = 'category'">Add category</a></p>
					<p><a href="#" data-bs-toggle="modal" data-bs-target="#hulaModal" v-on:click="chosenform = 'scopelevel'">Add level</a></p>
					<p><a href="#" data-bs-toggle="modal" data-bs-target="#hulaModal" v-on:click="chosenform = 'deleteuser'">Delete user</a></p>
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
					<AdminListProjects @project-chosen="updateProject" :leads='projectsdata' v-if="tab == 1" />
					<AdminListUsers :users='usersdata' v-if="tab == 2"  />
					<AdminListSkills :users='skillsdata' v-if="tab == 3"  />
				</div>
			</div>
		</div>
	</div>
</template>

<script>
	import VModal from '../components/VModal.vue'
	import FormProject from '../forms/FormProject.vue'
	import FormCreateSkill from '../forms/FormCreateSkill.vue'
	import FormSkillScope from '../forms/FormSkillScope.vue'
	import FormSkillCategory from '../forms/FormSkillCategory.vue'
	import FormSkillScopeLevel from '../forms/FormSkillScopeLevel.vue'
	import FormUserDelete from '../forms/FormUserDelete.vue'
	import AdminListProjects from '../components/AdminListProjects.vue'
	export default {
		name: 'Admin',
		data() {
			return {
				form_title: '',	
				tab: 1,
				chosenproject: {},
				chosenform: '',
			}
		},
		components: {
			VModal,
			FormProject,
			FormCreateSkill,
			FormSkillScope,
			FormSkillCategory,
			FormSkillScopeLevel,
			FormUserDelete,
			AdminListProjects,
		},
		methods: {
			updateProject (value) {
				console.log("updateProject fired")
				console.log(value)
				this.chosenform = 'project'
				this.chosenproject = value
				this.form_title = value.name
			}
		},
		computed: {
			modalComponent() {
				const components = {
					project: FormProject,
					skill: FormCreateSkill,
					scope: FormSkillScope,
					category: FormSkillCategory,
					scopelevel: FormSkillScopeLevel,
					deleteuser: FormUserDelete,
				}
				console.log(this.chosenform)
				return components[this.chosenform]
			}
		}
	}
</script>