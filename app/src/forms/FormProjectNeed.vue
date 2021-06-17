<template>
	<form v-on:submit="createProjectNeed">
		<h3>New need</h3>
		<label class="form-label">How many pros for this need?</label>
		<input type="number" aria-label="Number of pros" class="form-control mb-2" v-model.number="querydata_need.count_of_users">
		<label class="form-label">When does this need start?</label>
		<input type="date" aria-label="Date start" class="form-control mb-2" v-model="querydata_need.begin_time">
		{{ querydata_need.begin_time }}
		<label class="form-label">When does this need end?</label>
		<input type="date" aria-label="Date end" class="form-control mb-2" v-model="querydata_need.end_time">
		<label class="form-label">At what percentage?</label>
		<input type="number" aria-label="Percentage" class="form-control mb-2" v-model.number="querydata_need.percentage">
		<button type="submit" class="btn btn-gradient mb-1">Save</button>
	</form>
</template>

<script>
export default {
	name: 'ProjectNeed',
	data() {
		return {
			querydata_need: {
				id: '06ba4809-f20b-4687-945b-e033a6751fca',
				project_id: this.$store.state.chosenproject.id,
				count_of_users: Number,
				begin_time: "",
				end_time: "",
				percentage: Number,
				updated_by: this.$store.state.loggeduser.email
			},
		}
	},
	methods: {
		createProjectNeed: function() {
			this.querydata_need.begin_time = `${this.querydata_need.begin_time}T00:00:00`
			this.querydata_need.end_time = `${this.querydata_need.end_time}T00:00:00`
			fetch('/api/projectneeds', {
				method: 'POST',
				headers: {"Content-Type": "application/json"},
				credentials: 'include',
				body: JSON.stringify(this.querydata_need)
			})
			.catch((errors) => {
				console.log(errors);
			})
		},
	}
};
</script>