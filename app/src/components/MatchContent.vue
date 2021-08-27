<template>
	<div>
		<h4 class="h4">
			<a :href="`/app/user/${user_id}`">{{ user_first_name }} {{ user_last_name }}</a>
			<i class="bi-heart-fill mx-2"></i>
			<a :href="`/app/project/${project_id}`">{{ project_name }}</a>
		</h4>
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
					<tr v-for="match in matches" :key="match.idx">
						<td>{{ match.skill_label }}</td>
						<td>{{ match.required_index}}</td>
						<td>{{ match.user_index }}</td>
						<td>{{ match.required_years}}</td>
						<td>{{ match.user_years }}</td>
					</tr>
				</tbody>
			</table>
		</div>
		<a href="#" v-on:click.prevent="addOffer"><i class="bi-briefcase-fill"></i></a>
    </div>
</template>

<script>
	export default {
		name: 'MatchContent',

		props: {
			user_id: {
				type: String,
				required: true,
			},
			user_first_name: {
				type: String,
				required: true,
			},
			user_last_name: {
				type: String,
				required: true,
			},
			project_id: {
				type: String,
				required: true,
			},
			project_name: {
				type: String,
				required: true,
			},
			matches: {
				type: Array,
				required: true,
			},
		},

		methods: {
			async addOffer() {
				const offer = await this.$api.offers.save({
					user_id: this.user_id,
					project_id: this.project_id,
					sold: false,
					comments: '',
				})

				if (offer) {
					this.$emit('offer-saved')
					this.$flashMessage.show({
						type: 'success',
						title: 'Offer added.',
						time: 5000,
					})
				}
			}
		},
	}
</script>