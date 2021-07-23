<template>
	<v-form v-on:submit="onSubmit">
		<div class="mb-2 form-check">
			<label class="form-label">Hide me</label>
			<input type="checkbox" class="form-check-input" name="is_hidden" v-model="user.is_hidden" />
		</div>
		<div class="mb-2">
			<label class="form-label">First name</label>
			<error-message name="firstname" class="error"></error-message>
			<v-field
				name="firstname" 
				type="text" 
				placeholder="Mikki" 
				:rules="isRequired" 
				class="form-control" 
				v-model="user.firstname"
			></v-field>
		</div>
		<div class="mb-2">
			<label class="form-label">Last name</label>
			<error-message name="lastname" class="error"></error-message>
			<v-field
				name="lastname" 
				type="text" 
				placeholder="Hiiri" 
				:rules="isRequired" 
				class="form-control" 
				v-model="user.lastname"
			></v-field>
		</div>
		<div class="mb-2">
			<label class="form-label">CV</label>
			<error-message name="cv" class="error"></error-message>
			<v-field
				name="cv"
				type="file"
				class="form-control" 
				v-model="user.file"
			></v-field>
		</div>
		<button type="submit" class="btn btn-gradient">Save</button>
	</v-form>   
</template>

<script>
import { Field, Form, ErrorMessage } from 'vee-validate';
export default {
	name: 'UserBasicInfo',
	props: {	
		user: {}
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
		onSubmit() {
			this.$emit('formsent', this.user);
		},
	}
};
</script>