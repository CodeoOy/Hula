<template>
	<v-form v-on:submit="createUpdateUserReservation">
		<div class="mb-2">
			<label class="form-label">Description</label>
			<error-message name="description" class="error"></error-message>
			<v-field
				v-model="formData.description"
				:rules="isRequired"
				as="input"
				type="text"
				name="description"
				class="form-control"
				aria-label="description" 
			></v-field>
		</div>
		<div class="mb-2">
			<label class="form-label">When does this reservation start?</label>
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
				description: this.chosenReservation.description || '',
				begin_time: this.chosenReservation.begin_time || '',
				end_time: this.chosenReservation.end_time || '',
				percentage: this.chosenReservation.percentage || '',
				user_id: this.userID,
			}
		}
	},
	components: {
		'VForm': Form,
		'VField': Field,
		ErrorMessage
	},
	props: {
		url: '',
		method: '',
		userID: '',
		chosenReservation: {},
	},
	methods: {
		isRequired(value) {
			return value ? true : 'This field is required';
		},
		createUpdateUserReservation() {
			const formData = { ...this.formData }
			if (formData.end_time == '') delete formData.end_time
			fetch(this.url, {
				method: this.method,
				headers: {"Content-Type": "application/json"},
				credentials: 'include',
				body: JSON.stringify(formData)
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
};
</script>