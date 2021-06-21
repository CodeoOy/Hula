<template>
	<div>
		<h2>Projects</h2>
		<transition name="fadeHeight">
			<table class="table table-dark table-striped text-light">
				<thead>
					<tr>
						<th scope="col">#</th>
						<th scope="col">Project name</th>
						<th scope="col">needs</th>
						<th scope="col">Actions</th>
					</tr>
				</thead>
				<tbody>
					<tr v-for="(project, index) in this.$store.state.projects" :key="project.id">
						<th scope="row">{{ index + 1 }}</th>
						<td><router-link :to="{ name: 'page-project', params: { id: project.id}}">{{ project.name }}</router-link></td>
						<td>
							<p v-for="need in project.needs" :key="need.id">{{ need.id }}</p>
						</td>
						<td>
							<!--<a href="#" 
								:data-project-id="project.id" 
								:data-project-name="project.name" 
								data-bs-toggle="modal" 
								data-bs-target="#hulaModal" 
								v-on:click.prevent="this.chooseProject(project)"
							>Edit</a>-->
							<a href="#"
								v-on:click.prevent="this.deleteProject(project.id)"
							>Delete
							</a>
						</td>
					</tr>
				</tbody>
			</table>
		</transition>
	</div>
</template>

<script>
	export default {
		name: 'AdminListProjects',
		data() {
			return {
				projects: this.$store.state.projects,
			}
		},
		methods: {
			chooseProject(project) {
				this.$store.commit('setChosenProject', project.id)
				this.$emit('projectChosen', project.name)
			},
			deleteProject(id) {
				fetch(`/api/projects/${id}`, {
					method: 'DELETE',
					headers: {"Content-Type": "application/json"},
					credentials: 'include'
				})
				.catch(() => {
					throw new Error('Project not deleted');
				})
			}
		}
	}
</script>