<template>
	<VForm @submit='onSubmit' class='vstack gap-2'>

		<div class='form-check'>
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

		<div>
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

		<div class='mt-label'>
			<button type='submit' class='btn btn-primary gradient float-end'>Save</button>
		</div>
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