<template>
	<VForm @submit='onSubmit' v-slot='{ errors }'>

		<div class='mb-2 form-check'>
			<label for='sold' class='form-label'>Sold</label>
			<VField
				v-model='form.sold'
				:value='true'
				:unchecked-value='false'
				type='checkbox'
				id='sold'
				name='sold'
				class='form-check-input'
			/>
		</div>

		<div class='mb-2'>
			<label for='comments' class='form-label'>Comments</label>
			<error-message name='comments' class='invalid-feedback shake'></error-message>
			<VField
				v-model='form.comments'
				type='text'
				id='comments'
				name='comments'
				label='Comments'
				aria-label='Comments'
				class='form-control'
			/>
		</div>

		<button type='submit' class='btn btn-primary gradient mb-1'>Save</button>
	</VForm>
</template>

<script>
	export default {
		name: 'FormOffer',

		props: {
			id: {
				type: String,
				required: true, // At the moment we can't create offers with a form
			},
			sold: {
				type: Boolean,
				default: false,
			},
			comments: String,
		},

		data() {
			return {
				form: { ...this.$props },
			}
		},

		methods: {
			async onSubmit() {
				const offer = await this.$api.offers.save(this.form)
				if (offer) this.$emit('success', offer)
			},
		}
	}
</script>