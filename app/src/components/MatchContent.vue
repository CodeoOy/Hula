<template>
	<div>
		{{ chosenMatch }}
		{{ chosenProject }}
        <table class="table table-dark table-striped text-light">
			<thead>
				<tr>
					<th></th>
					<th scope="col">{{ chosenMatch.projectname }}</th>
					<th scope="col">{{ chosenMatch.firstname }} {{ chosenMatch.lastname }}</th>
				</tr>
			</thead>
			<tbody>
				<tr>
					<td>{{ chosenMatch.skillname }}</td>
					<td>{{ chosenMatch.required_minyears }}</td>
					<td>{{ chosenMatch.user_years }}</td>
				</tr>
			</tbody>
		</table>
		<a href="#" v-on:click="addToOffers"><i class="bi-trash-fill me-2"></i></a>
    </div>
</template>

<script>
	export default {
		name: 'MatchContent',
		data() {
			return {
				skills: {},
				formData: {
					user_id: this.chosenMatch.user_id,
					project_id: this.chosenProject.id,
					sold: false,
					comments: 'Lol.',
				}
			}
		},
		props: {
			chosenMatch: {},
			chosenProject: {},
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