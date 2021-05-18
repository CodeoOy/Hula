<template>
	<div class="container mt-4">
		<Modal :showmodal='true'>
			<Skill />
		</Modal>
		<div class="row gx-4">
			<div class="col-md-4">
				<div class="p-3 mb-4 rounded-2 content-box bg-dark text-light">
					<h1>Welcome {{ this.$store.state.loggeduser.firstname }}!</h1>
					<p><a href="#" v-on:click="showModal = true">Add skill</a></p>
				</div>
			</div>
			<div class="col-md-8">
				<div class="p-3 mb-4 rounded-2 content-box bg-dark text-light">
					
				</div>
			</div>
		</div>
	</div>
</template>

<script>
	import Modal from '../components/Modal.vue'
	import Skill from '../forms/Skill.vue'
	import SkillScope from '../forms/SkillScope.vue'
	export default {
		name: 'Admin',
		data() {
			return {
				editing_skills: false,
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
			SkillScope
		},
	}
</script>