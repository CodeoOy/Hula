<template>
	<div class="container mt-4">
		<VModal :modalTitle="formTitle" :modalID="'SingleProject'">
			<component :is='modalComponent' :chosenNeed="chosenNeed"/>
		</VModal>
		<div class="row gx-4">
			<div class="col-md-4">
                <div class="p-3 mb-4 rounded-2 content-box bg-dark text-light">
                	<h2>{{ project.name }}</h2>
                </div>
			</div>
			<div class="col-md-8">
				<div class="p-3 mb-4 rounded-2 content-box bg-dark text-light">
					<div class="d-flex flex-row justify-content-between align-items-start">
						<h2>Needs</h2>
						<button class="btn btn-gradient" data-bs-toggle="modal" data-bs-target="#hulaModalSingleProject" v-on:click="formTitle = 'Add need', chosenForm = 'need', chosenNeed = {}">Add need</button>
					</div>
					<div class="mt-3" v-for="need in project.needs" :key="need.id">
						<hr />
						<div class="d-flex flex-row justify-content-between align-items-baseline mb-3">
							<h5>{{ need.count_of_users}} from {{ need.begin_time }} at percentage: {{ need.percentage}}</h5>
							<div class="btn-group" role="group" aria-label="Need actions">
								<a href="#" class="me-2" data-bs-toggle="modal" data-bs-target="#hulaModalSingleProject" v-on:click="chosenNeed = need, formTitle = 'Edit need', chosenForm = 'need'">Edit</a>
								<a href="#" class="me-2" data-bs-toggle="modal" data-bs-target="#hulaModalSingleProject" v-on:click="chosenNeed = need, formTitle = 'New skill', chosenForm = 'skill'">Add skill</a>
								<a href="#" v-on:click="deleteNeed(need.id)">Delete</a>
							</div>
						</div>
						<table class="table table-dark table-striped text-light mb-4">
							<thead>
								<tr>
									<th scope="col">Skill</th>
									<th scope="col">Min level</th>
									<th scope="col">Min years</th>
									<th scope="col">Max years</th>
									<th scope="col">Actions</th>
								</tr>
							</thead>
							<tbody>
								<tr v-for="skill in need.skills" :key="skill.id">
									<td>{{ skill.id }}</td>
									<td>{{ skill.level }}</td>
									<td>{{ skill.min_years }}</td>
									<td>{{ skill.max_years }}</td>
									<td>
										<a href="#" class="me-2">Edit</a>
										<a href="#" v-on:click="deleteSkill(skill.id)">Delete</a>
									</td>
								</tr>
							</tbody>
						</table>
					</div>
				</div>
			</div>
		</div>
	</div>
</template>

<script>
	import VModal from '../components/VModal.vue'
	import FormProjectNeed from '../forms/FormProjectNeed.vue'
	import FormProjectNeedSkill from '../forms/FormProjectNeedSkill.vue'
	export default {
		name: 'Project',
		data() {
			return {
				chosenNeed: {},
				formTitle: '',
				chosenForm: ''
			}
		},
		components: {
			VModal,
			FormProjectNeed,
			FormProjectNeedSkill,
		},
		computed: {
			project() {
				return this.$store.state.chosenproject
			},
			modalComponent() {
				const components = {
					need: FormProjectNeed,
					skill: FormProjectNeedSkill,
				}
				return components[this.chosenForm]
			}
		},
		methods: {
			deleteNeed(id) {
				fetch(`/api/projectneeds/${id}`, {method: 'DELETE'})
				.then(response => { 
					if (response.ok) {
						this.$flashMessage.show({
							type: 'success',
							title: 'Need removed',
							time: 1000
						});
						this.$store.commit('setChosenProject', this.$route.params.id)
					}
				})    
				.catch((errors) => {
					console.log(errors);
				})
			},
			deleteSkill(id) {
				fetch(`/api/projectskills/${id}`, {method: 'DELETE'})
				.then(response => { 
					if (response.ok) {
						this.$flashMessage.show({
							type: 'success',
							title: 'Skill removed',
							time: 1000
						});
						this.$store.commit('setChosenProject', this.$route.params.id)
					}
				})    
				.catch((errors) => {
					console.log(errors);
				})
			}
		},
        mounted () {
			this.$store.commit('setChosenProject', this.$route.params.id)
		}
	}
</script>