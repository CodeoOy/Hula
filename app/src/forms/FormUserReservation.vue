<template>
	<v-form v-on:submit="createUpdateUserReservation">
		<h2 v-if="'id' in this.chosenReservation">{{ chosenReservation.id }}</h2>
		<h2 v-else>New reservation</h2>
		<div class="mb-2">
			<label class="form-label">When does this reservation start?</label>
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
			<label class="form-label">When does this reservation end?</label>
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
	name: 'UserReservation',
	data() {
		return {
			formData: {
				begin_time: this.chosenReservation.begin_time || '',
				end_time: this.chosenReservation.end_time || '',
				percentage: this.chosenReservation.percentage || '',
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
		createUpdateUserReservation() {
			let chosenMethod = 'PUT'
			let fetchPath = '/api/userreservations/' + this.chosenReservation.id
			if ('isNew' in this.chosenReservation) {
				chosenMethod = 'POST'
				fetchPath = '/api/userreservations'
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
		chosenReservation: {},
	},
};
</script>