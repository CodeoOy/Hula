<template>
	<v-form v-on:submit="createUpdateProjectNeed">
		<h2 v-if="'id' in this.chosenNeed">{{ chosenNeed.id }}</h2>
		<h2 v-else>New need</h2>
		{{ queryDataNeed }}
		<div class="mb-2">
			<label class="form-label">How many pros for this need?</label>
			<error-message name="users" class="error"></error-message>
			<v-field
				v-model.number="queryDataNeed.count_of_users"
				:rules="isRequired"
				as="input"
				type="number"
				name="users"
				class="form-select"
				aria-label="Number of pros" 
			></v-field>
		</div>
		<div class="mb-2">
			<label class="form-label">When does this need start?</label>
			{{ queryDataNeed.begin_time }}<br />
			<error-message name="begintime" class="error"></error-message>
			<v-field
				v-model="queryDataNeed.begin_time"
				:rules="isRequired"
				as="input"
				type="date"
				name="begintime"
				class="form-select"
				aria-label="Date start" 
			></v-field>
		</div>
		<div class="mb-2">
			<label class="form-label">When does this need end?</label>
			{{ queryDataNeed.end_time }}<br />
			<error-message name="endtime" class="error"></error-message>
			<v-field
				v-model="queryDataNeed.end_time"
				as="input"
				type="date"
				name="endtime"
				class="form-select"
				aria-label="Date end" 
			></v-field>
		</div>
		<div class="mb-2">
			<label class="form-label">At what percentage?</label>
			<error-message name="percentage" class="error"></error-message>
			<v-field
				v-model.number="queryDataNeed.percentage"
				:rules="isRequired"
				as="input"
				type="number"
				name="percentage"
				class="form-select"
				aria-label="Percentage" 
			></v-field>
		</div>
		<button type="submit" class="btn btn-gradient mb-1">Save</button>
	</v-form>
</template>

<script>
import { Field, Form, ErrorMessage } from 'vee-validate';
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
	components: {
		'VForm': Form,
		'VField': Field,
		ErrorMessage
	},
	methods: {
		isRequired(value) {
			return value ? true : 'This field is required';
		},
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
				.then(() => {
					this.$router.go()
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