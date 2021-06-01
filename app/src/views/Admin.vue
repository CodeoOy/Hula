<template>
	<div class="container mt-4">
		<Modal :show_modal="show_modal" :modal_title="form_title">
			<Project v-if="form_title == 'New Project'"/>
			<Skill v-if="form_title == 'New Skill'"/>
			<SkillScope v-if="form_title == 'New Scope'"/>
			<SkillCategory v-if="form_title == 'New Category'"/>
			<SkillScopeLevel v-if="form_title == 'New Level'"/>
			<UserDelete v-if="form_title == 'Delete User'"/>
		</Modal>
		<div class="row gx-4">
			<div class="col-md-4">
				<div class="p-3 mb-4 rounded-2 content-box bg-dark text-light">
					<h1>Welcome {{ this.$store.state.loggeduser.firstname }}!</h1>
					<p><a href="#" data-bs-toggle="modal" data-bs-target="#hulaModal" v-on:click="show_modal = true, form_title = 'New Project'">Add project</a></p>
					<p><a href="#" data-bs-toggle="modal" data-bs-target="#hulaModal" v-on:click="show_modal = true, form_title = 'New Skill'">Add skill</a></p>
					<p><a href="#" data-bs-toggle="modal" data-bs-target="#hulaModal" v-on:click="show_modal = true, form_title = 'New Scope'">Add scope</a></p>
					<p><a href="#" data-bs-toggle="modal" data-bs-target="#hulaModal" v-on:click="show_modal = true, form_title = 'New Category'">Add category</a></p>
					<p><a href="#" data-bs-toggle="modal" data-bs-target="#hulaModal" v-on:click="show_modal = true, form_title = 'New Level'">Add level</a></p>
					<p><a href="#" data-bs-toggle="modal" data-bs-target="#hulaModal" v-on:click="show_modal = true, form_title = 'Delete User'">Delete user</a></p>
				</div>
			</div>
			<div class="col-md-8">
				<ul class="nav nav-tabs nav-dark">
					<li class="nav-item">
						<a class="nav-link">Projects</a>
					</li>
					<li class="nav-item">
						<a class="nav-link">Users</a>
					</li>
					<li class="nav-item">
						<a class="nav-link">Skills</a>
					</li>
				</ul>
				<div class="p-3 mb-4 rounded-2 content-box bg-dark text-light">
					
				</div>
			</div>
		</div>
		{{ form_title }}
	</div>
</template>

<script>
	import Modal from '../components/Modal.vue'
	import Project from '../forms/Project.vue'
	import Skill from '../forms/Skill.vue'
	import SkillScope from '../forms/SkillScope.vue'
	import SkillCategory from '../forms/SkillCategory.vue'
	import SkillScopeLevel from '../forms/SkillScopeLevel.vue'
	import UserDelete from '../forms/UserDelete.vue'
	export default {
		name: 'Admin',
		data() {
			return {
				show_modal: false,
				form_title: '',			
			}
		},
		components: {
			Modal,
			Project,
			Skill,
			SkillScope,
			SkillCategory,
			SkillScopeLevel,
			UserDelete,
		},
	}
</script>