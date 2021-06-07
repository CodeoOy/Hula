<template>
	<div>
		<VModal :modal_title="chosenproject.name">
			<!--<Project :chosenproject="chosenproject.id"/>-->
			<FormProject :chosenproject="chosenproject" />
		</VModal>
		<h2>Projects</h2>
		{{ chosenproject.name }}<br />
		{{ this.$store.state.chosenproject.name }}
		<transition name="fadeHeight">
			<table class="table table-dark table-striped text-light">
				<thead>
					<tr>
						<th scope="col">#</th>
						<th scope="col">Project name</th>
						<th scope="col">id</th>
						<th scope="col">needs</th>
						<th scope="col">Actions</th>
					</tr>
				</thead>
				<tbody>
					<tr v-for="(project, index) in this.$store.state.projects" :key="project.id">
						<th scope="row">{{ index + 1 }}</th>
						<td>{{ project.name }}</td>
						<td>{{ project.id }}</td>
						<td>
							<p v-for="need in project.needs" :key="need.id">{{ need.id }}</p>
						</td>
						<td>
							<a href="#" 
								:data-project-id="project.id" 
								:data-project-name="project.name" 
								data-bs-toggle="modal" 
								data-bs-target="#hulaModal" 
								v-on:click.prevent="this.setProject(project)"
							>Edit</a>
							- Delete
						</td>
					</tr>
				</tbody>
			</table>
		</transition>
	</div>
</template>

<script>
	import VModal from './VModal.vue'
	import FormProject from '../forms/FormProject.vue'
	export default {
		name: 'AdminListProjects',
		data() {
			return {
				projects: this.$store.state.projects,
				chosenproject: {},
			}
		},
		components: {
			VModal,
			FormProject
		},
		methods: {
			setProject(project) {
				this.chosenproject = project
				console.log("setChosenProject fired")
				this.$store.commit('setChosenProject', this.chosenproject)
				this.$emit('projectChosen', this.chosenproject)
			}
		}
	}
</script>