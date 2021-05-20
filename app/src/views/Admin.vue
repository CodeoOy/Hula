<template>
	<div class="container mt-4">
		<Modal :showModal="showModal" :modalTitle="formTitle">
			<Skill v-if="formTitle == 'New Skill'"/>
			<SkillScope v-if="formTitle == 'New Scope'"/>
			<SkillCategory v-if="formTitle == 'New Category'"/>
			<SkillScopeLevel v-if="formTitle == 'New Level'"/>
		</Modal>
		<div class="row gx-4">
			<div class="col-md-4">
				<div class="p-3 mb-4 rounded-2 content-box bg-dark text-light">
					<h1>Welcome {{ this.$store.state.loggeduser.firstname }}!</h1>
					<p><a href="#" v-on:click="showModal = true, formTitle = 'New Skill'">Add skill</a></p>
					<p><a href="#" v-on:click="showModal = true, formTitle = 'New Scope'">Add scope</a></p>
					<p><a href="#" v-on:click="showModal = true, formTitle = 'New Category'">Add category</a></p>
					<p><a href="#" v-on:click="showModal = true, formTitle = 'New Level'">Add level</a></p>
				</div>
			</div>
			<div class="col-md-8">
				<div class="p-3 mb-4 rounded-2 content-box bg-dark text-light">
					
				</div>
			</div>
		</div>
		{{ formTitle }}
	</div>
</template>

<script>
	import Modal from '../components/Modal.vue'
	import Skill from '../forms/Skill.vue'
	import SkillScope from '../forms/SkillScope.vue'
	import SkillCategory from '../forms/SkillCategory.vue'
	import SkillScopeLevel from '../forms/SkillScopeLevel.vue'
	export default {
		name: 'Admin',
		data() {
			return {
				showModal: false,
				formTitle: '',
				skilldata: {
					label: '',
					category_id: '',
					email: this.$store.state.loggeduser.email,
				},
				categories: {}			
			}
		},
		components: {
			Modal,
			Skill,
			SkillScope,
			SkillCategory,
			SkillScopeLevel,
		},
	}
</script>