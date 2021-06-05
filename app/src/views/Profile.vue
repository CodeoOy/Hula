<template>
	<div class="container mt-4">
		<Modal :modal_title="form_title">
			<UserSkill v-if="form_title == 'Add Skill'"/>
		</Modal>
		<div class="row gx-4">
			<div class="col-md-4">
				<div class="p-3 mb-4 rounded-2 content-box bg-dark text-light">
					<h1>{{ user.firstname }} {{ user.lastname }}</h1>
					<p>{{ user.email }}</p>
					<p>{{ user.telephone }}</p>
					<p>{{ user.id }}</p>
					<img src="" alt="">
				</div>
			</div>
			<div class="col-md-8">
				<div class="p-3 mb-4 rounded-2 content-box bg-dark text-light">
					<h2>Professional profile</h2>
					<h3>Basic info</h3>
					<a href="#" v-on:click="editing_info = true">Edit your info</a>
					<p>Available: {{ user.available }}</p>
					<transition name="fadeHeight">
						<UserBasicInfo :user='user' v-if="editing_info == true" v-on:formsent="editing_info = false, updateUser()"/>
					</transition>
					<h3>Skills</h3>
					<table class="table table-dark table-striped text-light">
						<thead>
							<tr>
								<th scope="col">Skill</th>
								<th scope="col">Years</th>
								<th scope="col">Actions</th>
							</tr>
						</thead>
						<tbody>
							<tr v-for="skill in user.skills" :key="skill.id">
								<td>{{ skill.skill_label }}</td>
								<td>{{ skill.years }}</td>
								<td>Edit - Delete</td>
							</tr>
						</tbody>
					</table>
					<p><a href="#" v-on:click="form_title = 'Add Skill'" data-bs-toggle="modal" data-bs-target="#hulaModal">Add skill</a></p>
				</div>
			</div>
		</div>
	</div>
</template>

<script>
	import VModal from '../components/VModal.vue'
	import FormAddSkill from '../forms/FormAddSkill.vue'
	import FormUserBasicInfo from '../forms/FormUserBasicInfo.vue'
	export default {
		name: 'Profile',
		data() {
			return {
				show_modal: false,
				form_title: '',
				user: {},
				editing_info: false,
			}
		},
		components: {
			FormUserBasicInfo,
			FormAddSkill,
			VModal,
		},
		methods: {
			updateUser: function() { // This is not working. Make the user update happen in some other way.
				fetch(`/api/users/${this.user.id}`, {
					method: 'PUT',
					headers: {"Content-Type": "application/json"},
					credentials: 'include',
					body: JSON.stringify(this.user)
				})
				.then(() => {
					this.$store.commit('setUser', this.user.id)
				})
			}
		},
		mounted() {
			this.user = this.$store.state.loggeduser
		}
	}
</script>