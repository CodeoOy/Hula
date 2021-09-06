<template>
	<VForm v-on:submit='onSubmit'>

		<div class='mb-2'>
			<label for='description' class='form-label'>Description</label>
			<ErrorMessage name='description' class='error' />
			<VField
				v-model='form.description'
				rules='required'
				type='text'
				id='description'
				name='description'
				label='Description'
				aria-label='Description'
				class='form-control'
			/>
		</div>

		<div class='mb-2'>
			<label class='form-label'>Start date</label>
			<ErrorMessage name='begin_time' class='error' />
			<VField
				v-model='form.begin_time'
				rules='required|date'
				type='date'
				id='begin_time'
				name='begin_time'
				label='Start date'
				aria-label='Start date'
				class='form-control'
			/>
		</div>

		<div class='mb-2'>
			<label class='form-label'>End date</label>
			<ErrorMessage name='end_time' class='error' />
			<VField
				v-model='form.end_time'
				rules='required|date|afterDate:begin_time'
				type='date'
				id='end_time'
				name='end_time'
				label='End date'
				aria-label='End date'
				class='form-control'
			/>
		</div>

		<div class='mb-2'>
			<label for='percentage' class='form-label'>Percentage</label>
			<ErrorMessage name='percentage' class='error' />
			<VField
				v-model.number='form.percentage'
				rules='required'
				type='number'
				name='percentage'
				id='percentage'
				label='Percentage'
				aria-label='Percentage'
				class='form-control'
			/>
		</div>

		<button type='submit' class='btn btn-gradient mb-1'>Save</button>
	</VForm>
</template>

<script>
	export default {
		name: 'FormUserReservation',

		props: {
			id: {
				type: String,
				default: undefined,
			},
			user_id: {
				type: String,
				required: true,
			},
			description: String,
			begin_time: String,
			end_time: String,
			percentage: Number,
		},

		data() {
			return {
				form: { ...this.$props },
			}
		},

		methods: {
			async onSubmit() {
				for (const prop in this.form) if (this.form[prop] == '') this.form[prop] = undefined
				const reservation = await this.$api.users.reservations.save(this.form)
				if (reservation) this.$emit('success', reservation)
			},
		},
	}
</script>