<template>
	<v-form v-on:submit="createUpdateProjectNeed">
		<h2 v-if="'id' in this.chosenNeed">{{ chosenNeed.id }}</h2>
		<h2 v-else>New need</h2>
		<div class="mb-2">
			<label class="form-label">How many pros for this need?</label>
			<error-message name="users" class="error"></error-message>
			<v-field
				v-model.number="formData.count_of_users"
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
			{{ formData.begin_time }}<br />
			<error-message name="begintime" class="error"></error-message>
			<v-field
				v-model="formData.begin_time"
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
			{{ formData.end_time }}<br />
			<error-message name="endtime" class="error"></error-message>
			<v-field
				v-model="formData.end_time"
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
				v-model.number="formData.percentage"
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
	data() {
		return {
			formData: {
				project_id: this.$route.params.id,
				count_of_users: this.chosenNeed.count_of_users || '',
				begin_time: this.chosenNeed.begin_time.toString().replace('T00:00:00', '')  || '',
				end_time: this.chosenNeed.end_time.toString().replace('T00:00:00', '')  || '',
				percentage: this.chosenNeed.percentage || '',
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
			this.locked = true
			let chosenMethod = 'PUT'
			let fetchPath = '/api/projectneeds/' + this.chosenNeed.id
			if ('isNew' in this.chosenNeed) {
				chosenMethod = 'POST'
				fetchPath = '/api/projectneeds'
			}
			delete this.formData.skills
			this.formData.begin_time = `${this.formData.begin_time}T00:00:00` // TODO: Find a less hacky solution
			if (this.formData.end_time.length) {
				this.formData.end_time = `${this.formData.end_time}T00:00:00`
			}
			console.log(this.formData.begin_time)
			fetch(fetchPath, {
				method: chosenMethod,
				headers: {"Content-Type": "application/json"},
				credentials: 'include',
				body: JSON.stringify(this.formData)
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
	/*
	computed: {
		queryData: {
			get () {
				if (!this.locked) {
					var tempBeginTime = this.chosenNeed.begin_time.toString().replace('T00:00:00', '') 
					var tempEndTime = this.chosenNeed.end_time.toString().replace('T00:00:00', '') 
					this.chosenNeed.begin_time = tempBeginTime
					if (tempEndTime) {
						this.chosenNeed.end_time = tempEndTime
					} else {
						this.chosenNeed.end_time = null
					}
				}
				return this.chosenNeed
			}
		}
	}*/
};
</script>