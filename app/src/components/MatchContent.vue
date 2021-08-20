<template>
	<div>
		{{ chosenMatch }}
		<h4 class="h4">
			<a :href="`app/user/${chosenMatch.user_id}`">{{ chosenMatch.user_first_name }} {{ chosenMatch.user_last_name }}&nbsp;</a>
			<i class="bi-heart-fill me-2"></i>
			<a :href="`app/project/${chosenMatch.project_id}`">{{ projectName }}</a>
		</h4>
        <table class="table table-dark table-striped text-light">
			<thead>
				<tr>
					<th scope="col">User</th>
					<th scope="col">Project index</th>
					<th scope="col">User index</th>
					<th scope="col">Project years</th>
					<th scope="col">User years</th>
				</tr>
			</thead>
			<tbody>
				<tr v-for="skill in chosenMatch.matches" :key="skill.idx">
					<td>{{ skill.skill_label }}</td>
					<td>{{ skill.required_index}}</td>
					<td>{{ skill.user_index }}</td>
					<td>{{ skill.required_years}}</td>
					<td>{{ skill.user_years }}</td>
				</tr>
			</tbody>
		</table>
		<a href="#" v-on:click="addToOffers"><i class="bi-briefcase-fill me-2"></i></a>
    </div>
</template>

<script>
	export default {
		name: 'MatchContent',
		data() {
			return {
				skills: {},
				formData: {
					user_id: this.chosenMatch.user_id || '',
					project_id: this.chosenMatch.project_id|| '',
					sold: false,
					comments: 'Lol.',
				}
			}
		},
		props: {
			chosenMatch: {},
			projectName: '',
		},
		methods: {
			addToOffers() {
				fetch('/api/offers', {
					method: 'POST',
					headers: {"Content-Type": "application/json"},
					credentials: 'include',
					body: JSON.stringify(this.formData)
				})
				.then(response => { 
					return (response.status >= 200 && response.status <= 299) ? response.json() : this.$store.commit('errorHandling', response)
				})
				.then(response => { 
					this.$emit('formSent');
				})    
				.catch((errors) => {
					this.$store.commit('errorHandling', errors)
				})
			}
		}
	}
</script>