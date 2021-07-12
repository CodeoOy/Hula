<template>
	<v-form v-on:submit="deleteItem">
		<button type="submit" class="btn btn-gradient mb-1">Submit</button>
	</v-form> 
</template>

<script>
import { Field, Form, ErrorMessage } from 'vee-validate';
export default {
	name: 'FormConfirmAction',
	props: {
		url: '',
		method: '',
	},
	components: {
		'VForm': Form,
		'VField': Field,
		ErrorMessage
	},
	methods: {
		deleteItem() {
			fetch(this.url, {method: this.method})
			.then(response => { 
				return (response.status >= 200 && response.status <= 299) ? response : this.$store.commit('errorHandling', response)
			})
			.then(() => {
				this.$emit('formSent')
			})  
			.catch((errors) => {
				this.$store.commit('errorHandling', errors)
			})
		},
	}
};
</script>