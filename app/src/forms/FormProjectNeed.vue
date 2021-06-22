<template>
	<form v-on:submit="createUpdateProjectNeed">
		<h3 v-if="'id' in this.chosenNeed">{{ chosenNeed.id }}</h3>
		<h3 v-else>New need</h3>
		{{ queryDataNeed }}
		<label class="form-label">How many pros for this need?</label>
		<input type="number" aria-label="Number of pros" class="form-control mb-2" v-model.number="queryDataNeed.count_of_users">
		<label class="form-label">When does this need start?</label>
		<input type="date" aria-label="Date start" class="form-control mb-2" v-model="queryDataNeed.begin_time">
		{{ queryDataNeed.begin_time }}<br />
		<label class="form-label">When does this need end?</label>
		<input type="date" aria-label="Date end" class="form-control mb-2" v-model="queryDataNeed.end_time">
		{{ queryDataNeed.end_time }}<br />
		<label class="form-label">At what percentage?</label>
		<input type="number" aria-label="Percentage" class="form-control mb-2" v-model.number="queryDataNeed.percentage">
		<button type="submit" class="btn btn-gradient mb-1">Save</button>
	</form>
</template>

<script>
export default {
	name: 'ProjectNeed',
	data() {
		return {
			queryDataNeed: {
				id: '06ba4809-f20b-4687-945b-e033a6751fca',
				project_id: this.$store.state.chosenproject.id,
				count_of_users: Number,
				begin_time: "",
				end_time: "",
				percentage: Number,
				updated_by: this.$store.state.loggeduser.email
			}
		}
	},
	methods: {
		createUpdateProjectNeed() {
			this.queryDataNeed.begin_time = `${this.queryDataNeed.begin_time}T00:00:00` // TODO: This breaks the api call if user doesn't change the dates
			this.queryDataNeed.end_time = `${this.queryDataNeed.end_time}T00:00:00` // So maybe use moment.js?
			if ('id' in this.chosenNeed) {
				fetch(`/api/projectneeds/${this.chosenNeed.id}`, {
					method: 'PUT',
					headers: {"Content-Type": "application/json"},
					credentials: 'include',
					body: JSON.stringify(this.queryDataNeed)
				})
				.catch((errors) => {
					console.log(errors);
				})
			} else {
				fetch('/api/projectneeds', {
					method: 'POST',
					headers: {"Content-Type": "application/json"},
					credentials: 'include',
					body: JSON.stringify(this.queryDataNeed)
				})
				.catch((errors) => {
					console.log(errors);
				})
			}
		},
	},
	props: {
		chosenNeed: {}
	},
	watch: {
		'chosenNeed'() {
			if ('id' in this.chosenNeed) {
				delete this.chosenNeed.skills
				this.queryDataNeed = this.chosenNeed
			}
		},
	}
};
</script>