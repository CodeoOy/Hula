<template>
	<div>
		<Modal :show_modal="show_modal" :modal_title="chosenproject.name">
			<Project :chosenproject="chosenproject.id"/>
		</Modal>
		<h2>Projects</h2>
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
							<a href="#" :data-project-id="project.id" :data-project-name="project.name" data-bs-toggle="modal" data-bs-target="#hulaModal" v-on:click.prevent="linkToEdit($event)">Edit</a>
							- Delete
						</td>
					</tr>
				</tbody>
			</table>
		</transition>
	</div>
</template>

<script>
	import Modal from '../components/Modal.vue'
	import Project from '../forms/Project.vue'
	export default {
		name: 'AdminListProjects',
		data() {
			return {
				projects: this.$store.state.projects,
				chosenproject: {
					id: '',
					name: '',
				},
				show_modal: false
			}
		},
		components: {
			Modal,
			Project
		},
		methods: {
			linkToEdit(event) {
				let element = event.currentTarget		
				this.chosenproject.id = element.getAttribute('data-project-id');
				this.chosenproject.name = element.getAttribute('data-project-name');
				this.show_modal = true
			}
		}
	}
</script>