<template>
	<v-form v-on:submit="saveOffer">
		<div class="mb-2 form-check">
			<label class="form-label">Sold</label>
			<input type="checkbox" class="form-check-input" name="sold" v-model="formData.sold" />
		</div>
		<div class="mb-2">
			<label class="form-label">Comments</label>
			<error-message name="name" class="error"></error-message>
			<v-field
				v-model="formData.comments"
				:rules="isRequired"
				as="input"
				type="text"
				name="comments"
				class="form-control"
				aria-label="Offer comments"
			></v-field>
		</div>
		<button type="submit" class="btn btn-gradient mb-1">Save</button>
	</v-form> 
</template>

<script>
import { Field, Form, ErrorMessage } from 'vee-validate';
export default {
	name: 'FormOffer',
	data() {
		return {
			formData: {
				id: this.chosenOfferID || undefined,
				sold: false,
				comments: '',
			},
		}
	},
	components: {
		'VForm': Form,
		'VField': Field,
		ErrorMessage,
	},
	props: {
		chosenOfferID: '',
	},	
	methods: {
		isRequired(value) {
			return value ? true : 'This field is required';
		},
		async saveOffer() {
			console.log(this.formData)
			const offer = await this.$api.offers.save(this.formData)
			if (offer) this.$emit('formSent')
		},
	}
};
</script>