<template>
	<div>
		<h4 class="h4">
			<router-link :to='{ name: "user", params: { id: user_id } }'>{{ user_first_name }} {{ user_last_name }}</router-link>
			<i class="bi-heart-fill mx-2"></i>
			<router-link :to='{ name: "project", params: { id: project_id } }'>{{ project_name }}</router-link>
		</h4>
		<div class="table-responsive">
			<table class="table table-striped" :class='$colorScheme.table'>
				<thead>
					<tr>
						<th scope="col">Skill</th>
						<th scope="col" class='text-center'>Required level</th>
						<th scope="col" class='text-center'>User level</th>
						<th scope="col" class='text-center'>Required years</th>
						<th scope="col" class='text-center'>User years</th>
					</tr>
				</thead>
				<tbody>
					<tr v-for="skill in skills" :key="skill.idx">
						<td>{{ skill.skill_label }}</td>
						<td class='text-center'>{{ skill.required_index}}</td>
						<td class='text-center'>{{ skill.user_index }}</td>
						<td class='text-center'>{{ skill.required_years}}</td>
						<td>{{ skill.user_years }}</td>
					</tr>
				</tbody>
			</table>
		</div>
		<button class='btn btn-link' v-on:click="addOffer"><i class="bi-briefcase-fill"></i></button>
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
			skills: {
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