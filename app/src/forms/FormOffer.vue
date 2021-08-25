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
				v-model="formData.label"
				:rules="isRequired"
				as="input"
				type="text"
				name="name"
				class="form-control"
				aria-label="Skill name"
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
				sold: false,
				comments: '',
			},
		}
	},
	components: {
		'VForm': Form,
		'VField': Field,
		ErrorMessage
	},
	props: {
		chosenOffer: {}
	},	
	methods: {
		isRequired(value) {
			return value ? true : 'This field is required';
		},
		async saveOffer() {
			const offer = await this.$api.offers.save(this.formData)
			if (offer) this.$emit('formSent')
		},
	}
};
</script>