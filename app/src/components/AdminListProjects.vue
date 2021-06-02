<template>
	<div>
		<h2>Projects</h2>
		{{ projects }}
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
					<tr v-for="(project, index) in projects" :key="project.id">
						<th scope="row">{{ index + 1 }}</th>
						<td>{{ project.name }}</td>
						<td>{{ project.id }}</td>
						<td>{{ project.needs }}</td>
						<td>Edit - Delete</td>
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
				projects: this.$store.state.projects
			}
		},
		methods: {
		}
	}
</script>