<template>
	<v-form v-on:submit="createUpdateProjectNeed">
		<h2 v-if="'id' in this.chosenNeed">{{ chosenNeed.id }}</h2>
		<h2 v-else>New need</h2>
		{{ chosenNeed }}
		<div class="mb-2">
			<label class="form-label">How many pros for this need?</label>
			<error-message name="users" class="error"></error-message>
			<v-field
				v-model.number="chosenNeed.count_of_users"
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
			{{ chosenNeed.begin_time }}<br />
			<error-message name="begintime" class="error"></error-message>
			<v-field
				v-model="chosenNeed.begin_time"
				:rules="isRequired"
				as="input"
				type="date"
				name="begintime"
				class="form-control"
				aria-label="Date start" 
			></v-field>
		</div>
		<div class="mb-2">
			<label class="form-label">When does this need end?</label>
			{{ chosenNeed.end_time }}<br />
			<error-message name="endtime" class="error"></error-message>
			<v-field
				v-model="chosenNeed.end_time"
				as="input"
				type="date"
				name="endtime"
				class="form-control"
				aria-label="Date end" 
			></v-field>
		</div>
		<div class="mb-2">
			<label class="form-label">At what percentage?</label>
			<error-message name="percentage" class="error"></error-message>
			<v-field
				v-model.number="chosenNeed.percentage"
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
			let chosenMethod = 'POST'
			let fetchPath = '/api/projectneeds'
			if ('id' in this.chosenNeed) {
				chosenMethod = 'PUT'
				fetchPath = '/api/projectneeds/' + this.chosenNeed.id
			} else {
				this.chosenNeed.id = '00000000-0000-0000-0000-e033a6751fca'
			}
			delete this.chosenNeed.skills
			this.chosenNeed.begin_time = `${this.chosenNeed.begin_time}T00:00:00` // TODO: This breaks the api call if user doesn't change the dates
			this.chosenNeed.end_time = `${this.chosenNeed.end_time}T00:00:00` // So maybe use moment.js?
			fetch(fetchPath, {
				method: chosenMethod,
				headers: {"Content-Type": "application/json"},
				credentials: 'include',
				body: JSON.stringify(this.chosenNeed)
			})
			.then(() => {
				this.$router.go()
			})
			.catch((errors) => {
				console.log(errors);
			})
		},
	},
	props: {
		chosenNeed: {}
	},
};
</script>