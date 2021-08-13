<template>
	<div class="p-3 rounded-2 content-box bg-dark text-light">
		<h2>Pro search results</h2>
		<transition name="fadeHeight">
			<table v-if="users.length" class="table table-dark table-striped text-light">
				<thead>
					<tr>
						<th scope="col">#</th>
						<th scope="col">Name</th>
						<th scope="col">Has mandatory skill?</th>
						<th scope="col">All skills?</th>
						<th scope="col">Available?</th>
					</tr>
				</thead>
				<tbody>
					<tr v-for="(user, index) in users" :key="user.uid">
						<th scope="row">{{ index + 1 }}</th>
						<td>{{ user.first_name }} {{ user.last_name }}</td>
						<td>{{ user.has_mandatory }}</td>
						<td>{{ user.is_all_skills }}</td>
						<td>{{ user.is_available }}</td>
						<!--<a href="#" v-on:click="getUserData(user.uid)">{{user.firstname}} {{ user.lastname }}</a>-->
					</tr>
				</tbody>
			</table>
		</transition>
	</div>
</template>

<script>
	export default {
		name: 'ResultsPros',
		props: {
			users: {}
		}
	}
</script>