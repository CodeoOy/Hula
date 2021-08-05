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
				:rules="isRequiredDate"
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
				:rules="afterStartTime"
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
				begin_time: this.chosenNeed.begin_time || '',
				end_time: this.chosenNeed.end_time || '',
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
		isRequiredDate(value) {
			// check if value is a valid date
			const date = new Date(value);
			return date.toString() === 'Invalid Date' ? 'This field is required' : true;
		},
		afterStartTime(value) {
			return value ? true : 'End time must be after start time.';
		},
		createUpdateProjectNeed() {
			let chosenMethod = 'PUT'
			let fetchPath = '/api/projectneeds/' + this.chosenNeed.id
			if ('isNew' in this.chosenNeed) {
				chosenMethod = 'POST'
				fetchPath = '/api/projectneeds'
			}
			if (this.formData.end_time == '') {
				delete this.formData.end_time
			}
			delete this.formData.skills
			fetch(fetchPath, {
				method: chosenMethod,
				headers: {"Content-Type": "application/json"},
				credentials: 'include',
				body: JSON.stringify(this.formData)
			})
			.then(response => {
				if (response.status >= 200 && response.status <= 299) {
					this.$emit('formSent')
				} else {
					this.$store.commit('errorHandling', response)
				}
			})
			.catch((errors) => {
				this.$store.commit('errorHandling', errors)
			})
		},
	},
	props: {
		chosenNeed: {},
	},
};
</script>