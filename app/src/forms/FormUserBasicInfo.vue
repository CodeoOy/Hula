<template>	
	<v-form v-on:submit="editUserInfo">
		<div class="mb-2 form-check" v-if="'is_hidden' in chosenUser">
			<label class="form-label">Hide me</label>
			<input type="checkbox" class="form-check-input" name="is_hidden" v-model="formData.is_hidden" />
		</div>
		<div class="mb-2 form-check" v-if="this.$store.state.loggeduser.isadmin === true">
			<label class="form-label">Admin</label>
			<input type="checkbox" class="form-check-input" name="is_hidden" v-model="formData.is_admin" />
		</div>
		<div class="mb-2 form-check" v-if="this.$store.state.loggeduser.isadmin === true">
			<label class="form-label">Is employee</label>
			<input type="checkbox" class="form-check-input" name="is_hidden" v-model="formData.is_admin" />
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
				v-model="formData.firstname"
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
				v-model="formData.lastname"
			></v-field>
		</div>
		<div class="mb-2">
			<label class="form-label">Email</label>
			<error-message name="email" class="error"></error-message>
			<v-field
				name="email" 
				type="text"
				:rules="isRequired" 
				class="form-control" 
				v-model="formData.email"
			></v-field>
		</div>
		<div class="mb-2">
			<label class="form-label">CV</label>
			<error-message name="cv" class="error"></error-message>
			<v-field
				name="cv"
				type="file"
				class="form-control" 
				v-model="formData.file"
			></v-field>
		</div>
		<button type="submit" class="btn btn-gradient">Save</button>
	</v-form>   
</template>

<script>
import { Field, Form, ErrorMessage } from 'vee-validate';
export default {
	name: 'UserBasicInfo',
	data() {
		return {
			formData: {
				id: this.chosenUser.id,
				email: this.chosenUser.email || '',
				is_hidden: this.chosenUser.is_hidden || false,
				is_admin: this.chosenUser.is_admin || false,
				firstname: this.chosenUser.firstname || '',
				lastname: this.chosenUser.lastname || '',
			}
		}
	},
	props: {	
		chosenUser: {}
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
		editUserInfo() {
			fetch(`/api/users/${this.chosenUser.id}`, {
				method: 'PUT',
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
	}
};
</script>