<template>
	<div class="container mt-4">
		<VModal :modalTitle="formTitle" :modalID="'Profile'" v-on:updated-modal="chosenForm = '', chosenSkill = {}">
			<component 
				:is='modalComponent' 
				:url="url"
				:method="method"
				:chosenUser="user"
				:chosenSkill="chosenSkill"
				:chosenReservation="chosenReservation"
				:userID="user.id"
				v-on:form-sent="hideModalUpdate"
			/>
		</VModal>
		<div class="row gx-4">
			<div class="col-md-4">
				<div class="p-3 mb-4 rounded-2 content-box bg-dark text-light">
					<h1>{{ user.firstname }} {{ user.lastname }}</h1>
					<p>{{ user.email }}</p>
					<a 
						href="#"
						data-bs-toggle="modal" 
						data-bs-target="#hulaModalProfile" 
						v-on:click="formTitle = 'Edit user info', chosenForm = 'User', chosenUser = user, url=`/api/users/${user.id}`, method='PUT'"
					><i class="bi-pencil-fill me-2"></i></a>
					<a 
						href="#"
						data-bs-toggle="modal" 
						data-bs-target="#hulaModalProfile" 
						v-on:click="formTitle = 'Delete my profile', chosenForm = 'Delete', chosenUser = user, url=`/api/users/${user.id}`, method='DELETE'"
					><i class="bi-trash-fill me-2"></i></a>
				</div>
			</div>
			<div class="col-md-8">
				<div class="p-3 mb-4 rounded-2 content-box bg-dark text-light">
					<div class="d-flex flex-row justify-content-between align-items-start">
						<h3>Skills</h3>
						<button
							class="btn btn-gradient"
							v-on:click="formTitle = 'Add Skill', chosenSkill = {}, chosenForm = 'Skill', url = `/api/userskills/${user.id}`, method = 'POST'" 
							data-bs-toggle="modal" 
							data-bs-target="#hulaModalProfile"
						>Add skill</button>
					</div>
					<table class="table table-dark table-striped text-light">
						<thead>
							<tr>
								<th scope="col">Skill</th>
								<th scope="col">Level</th>
								<th scope="col">Years</th>
								<th scope="col">Actions</th>
							</tr>
						</thead>
						<tbody>
							<tr v-for="skill in user.skills" :key="skill.id">
								<td>{{ skill.skill_label }}</td>
								<td>{{ skill.skillscopelevel_id }}</td>
								<td>{{ skill.years }}</td>
								<td>
									<a 
										href="#"
										data-bs-toggle="modal"
										data-bs-target="#hulaModalProfile"
										v-on:click="formTitle = skill.skill_label, chosenForm = 'Skill', chosenSkill = skill, url=`/api/userskills/${skill.id}`, method='PUT'"
									><i class="bi-pencil-fill me-2"></i></a>
									<a
										href="#"
										data-bs-toggle="modal"
										data-bs-target="#hulaModalProfile" 
										v-on:click="formTitle = `Delete ${skill.skill_label}?`, chosenSkill = skill, chosenForm = 'Delete', url = `/api/userskills/${skill.id}`, method = 'DELETE'"
									><i class="bi-trash-fill me-2"></i></a>
								</td>
							</tr>
						</tbody>
					</table>
					<div class="d-flex flex-row justify-content-between align-items-start">
						<h3>Reservations</h3>
						<button
							class="btn btn-gradient"
							v-on:click="formTitle = 'Add Reservation', chosenReservation = {}, chosenForm = 'Reservation', url = `/api/userreservations/${user.id}`, method = 'POST'" 
							data-bs-toggle="modal" 
							data-bs-target="#hulaModalProfile"
						>Add reservation</button>
					</div>
					<table class="table table-dark table-striped text-light">
						<thead>
							<tr>
								<th scope="col">Reservation id</th>
								<th scope="col">From</th>
								<th scope="col">To</th>
								<th scope="col">Actions</th>
							</tr>
						</thead>
						<tbody>
							<tr v-for="reservation in user.reservations" :key="reservation.id">
								<td>{{ reservation.id }}</td>
								<td>{{ reservation.begin_time}}</td>
								<td>{{ reservation.end_time }}</td>
								<td>
									<a 
										href="#"
										data-bs-toggle="modal"
										data-bs-target="#hulaModalProfile"
										v-on:click="formTitle = reservation.reservation_label, chosenForm = 'reservation', chosenReservation = reservation, url=`/api/userreservations/${reservation.id}`, method='PUT'"
									><i class="bi-pencil-fill me-2"></i></a>
									<a
										href="#"
										data-bs-toggle="modal"
										data-bs-target="#hulaModalProfile" 
										v-on:click="formTitle = `Delete reservation?`, chosenReservation = reservation, chosenForm = 'Delete', url = `/api/userreservations/${reservation.id}`, method = 'DELETE'"
									><i class="bi-trash-fill me-2"></i></a>
								</td>
							</tr>
						</tbody>
					</table>
				</div>
			</div>
		</div>
	</div>
</template>

<script>
	import VModal from '../components/VModal.vue'
	import { Modal } from 'bootstrap'
	import FormUserSkill from '../forms/FormUserSkill.vue'
	import FormUserReservation from '../forms/FormUserReservation.vue'
	import FormSkill from '../forms/FormSkill.vue'
	import FormUserBasicInfo from '../forms/FormUserBasicInfo.vue'
	import FormConfirmAction from '../forms/FormConfirmAction.vue'
	export default {
		name: 'Profile',
		data() {
			return {
				formTitle: '',
				chosenForm: '',
				chosenUser: {},
				chosenSkill: {},
				chosenReservation: {},
				editingInfo: false,
				user: this.$store.state.loggeduser,
				url: '',
				method: '',
			}
		},
		components: {
			FormUserBasicInfo,
			FormUserSkill,
			FormUserReservation,
			FormSkill,
			FormConfirmAction,
			VModal,
		},
		methods: {
			hideModalUpdate() {
				this.checkProfile(this.$route.params.id)
				this.getUserReservations(this.$route.params.id)
				let modal = Modal.getInstance(document.querySelector('#hulaModalProfile'))
				modal.hide()
			},
			deleteSkill(id) {
				fetch(`/api/userskills/${id}`, {
					method: 'DELETE',
					headers: {"Content-Type": "application/json"},
					credentials: 'include',
					body: JSON.stringify(this.user)
				})
			},
			checkProfile(id) {
				fetch(`/api/users/${id}`, {method: 'GET'})
				.then(response => { 
					return (response.status >= 200 && response.status <= 299) ? response.json() : this.$store.commit('errorHandling', response)
				})
				.then(response => { 
					this.user = response;
				}) 
			},
			getUserReservations(id) {
				fetch(`/api/userreservations/${id}`, {method: 'GET'})
				.then(response => {
					return (response.status >= 200 && response.status <= 299) ? response.json() : this.$store.commit('errorHandling', response)
				})
				.then(response => {
					this.user.reservations = response;
				})
			}
		},
		computed: {
			modalComponent() {
				const components = {
					Delete: FormConfirmAction,
					User: FormUserBasicInfo,
					Skill: FormUserSkill,
					Reservation: FormUserReservation,
				}
				return components[this.chosenForm]
			},
		},
		mounted() {
			if (this.$route.params.id != this.$store.state.loggeduser.id) {
				if (this.$store.state.loggeduser.isadmin === true) {
					this.checkProfile(this.$route.params.id)
				} else {
					this.$router.push({name: 'page-error'})
				}
			}
			this.getUserReservations(this.$route.params.id)
		}
	}
</script>