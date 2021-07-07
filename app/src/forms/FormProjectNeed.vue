<template>
	<v-form v-on:submit="createUpdateProjectNeed">
		<h2 v-if="'id' in this.chosenNeed">{{ chosenNeed.id }}</h2>
		<h2 v-else>New need</h2>
		<div class="mb-2">
			<label class="form-label">How many pros for this need?</label>
			<error-message name="users" class="error"></error-message>
			<v-field
				v-model.number="queryData.count_of_users"
				:rules="isRequired"
				as="input"
				type="number"
				name="users"
				class="form-control"
				aria-label="Number of pros" 
			></v-field>
		</div>
		<div class="mb-2">
			<label class="form-label">When does this need start?</label>
			{{ queryData.begin_time }}<br />
			<error-message name="begintime" class="error"></error-message>
			<v-field
				v-model="queryData.begin_time"
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
			{{ queryData.end_time }}<br />
			<error-message name="endtime" class="error"></error-message>
			<v-field
				v-model="queryData.end_time"
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
				v-model.number="queryData.percentage"
				:rules="isRequired"
				as="input"
				type="number"
				name="percentage"
				class="form-control"
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
			if ('id' in this.queryData) {
				chosenMethod = 'PUT'
				fetchPath = '/api/projectneeds/' + this.queryData.id
			} else {
				this.queryData.id = '00000000-0000-0000-0000-e033a6751fca'
			}
			delete this.queryData.skills
			this.queryData.begin_time = `${this.queryData.begin_time}T00:00:00` 
			console.log(this.queryData.begin_time)// TODO: This breaks the api call if user doesn't change the dates
			this.queryData.end_time = `${this.queryData.end_time}T00:00:00` // So maybe use moment.js?
			console.log(this.queryData.begin_time)
			fetch(fetchPath, {
				method: chosenMethod,
				headers: {"Content-Type": "application/json"},
				credentials: 'include',
				body: JSON.stringify(this.queryData)
			})
			.then(() => {
				this.$emit('formSent')
			})
			.catch((errors) => {
				this.$store.commit('errorHandling', errors)
			})
		},
	},
	props: {
		chosenNeed: {}
	},
	computed: {
		queryData: {
			get () {
				var tempBeginTime = this.chosenNeed.begin_time.toString().replace('T00:00:00', '') 
				var tempEndTime = this.chosenNeed.end_time.toString().replace('T00:00:00', '') 
				this.chosenNeed.begin_time = tempBeginTime
				if (tempEndTime) {
					this.chosenNeed.end_time = tempEndTime
				}
				return this.chosenNeed
			}
		}
	}
};
</script>