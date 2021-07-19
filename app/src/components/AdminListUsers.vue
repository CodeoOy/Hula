<template>
	<div>
		<VModal :modalTitle="formTitle" :modalID="'Users'" v-on:updated-modal="chosenForm = ''">
			<component 
				:is='modalComponent' 
				:chosenUser="chosenUser" 
				:url="url"
				:method="method"
				v-on:form-sent="hideModalUpdate"
			/>
		</VModal>
		<div class="d-flex flex-row justify-content-between align-items-start">
			<h2>Users</h2>
			<div>
				<!--<AutoComplete :suggestions="this.$store.state.users" :selection.sync="userName" :placeholder="'filter Users'"></AutoComplete>-->
				<button
					class="btn btn-gradient"
					data-bs-toggle="modal"
					data-bs-target="#hulaModalUsers"
					v-on:click="formTitle = 'New User', chosenForm = 'CreateUser', chosenUser = {}, url='/api/users', method='POST'"
				>Invite a user</button>
			</div>
		</div>
	</div>
</template>

<script>
	import VModal from '../components/VModal.vue'
	import { Modal } from 'bootstrap'
	export default {
		name: 'AdminListUsers',
		data() {
			return {
				formTitle: '',
				chosenForm: '',
				url: '',
				method: '',
			}
		},
		methods: {
			hideModalUpdate() {
				this.getSkillCategories()
				this.getAllSkills()
				let modal = Modal.getInstance(document.querySelector('#hulaModalSkills'))
				modal.hide()
			}
		},
		components: {
			VModal,
		},
	}
</script>