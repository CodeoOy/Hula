<template>
	<div>
		<h4 class="h4">
			<a :href="`app/user/${chosenMatch.user_id}`">{{ chosenMatch.user_first_name }} {{ chosenMatch.user_last_name }}&nbsp;</a>
			<i class="bi-heart-fill me-2"></i>
			<a :href="`app/project/${chosenMatch.project_id}`">{{ projectName }}</a>
		</h4>
		{{ chosenMatch }}<br />
		{{ projectName}}
		<div class="table-responsive">
			<table class="table table-dark table-striped text-light">
				<thead>
					<tr>
						<th scope="col">Skill</th>
						<th scope="col">Required level</th>
						<th scope="col">User level</th>
						<th scope="col">Required years</th>
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
		</div>
		<a href="#" v-on:click="addToOffers"><i class="bi-briefcase-fill me-2"></i></a>
    </div>
</template>

<script>
	export default {
		name: 'MatchContent',
		data() {
			return {
				skills: {},
			}
		},
		props: {
			chosenMatch: {},
			projectName: '',
		},
		methods: {
			addToOffers() {
				const offer = this.$api.offers.save({
					user_id: this.chosenMatch.user_id || '',
					project_id: this.chosenMatch.project_id || '',
					sold: false,
					comments: '',
				})

				if (offer) {
					this.$emit('formSent')
					this.$flashMessage.show({
						type: 'success',
						title: 'Offer added.',
						time: 5000,
					})
				}
			}
		}
	}
</script>