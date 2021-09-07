<template>
	<VForm v-on:submit='onSubmit'>

		<div class='mb-2'>
			<label for='label' class='form-label'>Label</label>
			<ErrorMessage name='label' class='error' />
			<VField
				v-model='form.label'
				rules='required'
				type='text'
				name='label'
				id='label'
				label='Label'
				aria-label='Label'
				class='form-control'
			/>
		</div>

		<div class='mb-2'>
			<label for='count_of_users' class='form-label'>Number of positions</label>
			<ErrorMessage name='count_of_users' class='error' />
			<VField
				v-model.number='form.count_of_users'
				rules='required'
				type='number'
				name='count_of_users'
				id='count_of_users'
				label='Number of positions'
				aria-label='Number of positions'
				class='form-control'
			/>
		</div>

		<div class='mb-2'>
			<label for='begin_time' class='form-label'>Start date</label>
			{{ begin_time }}<br />
			<ErrorMessage name='begin_time' class='error' />
			<VField
				v-model='form.begin_time'
				rules='required|date'
				type='date'
				name='begin_time'
				id='begin_time'
				label='Start date' 
				aria-label='Start date' 
				class='form-control'
			/>
		</div>

		<div class='mb-2'>
			<label for='end_time' class='form-label'>End date</label>
			{{ end_time }}<br />
			<ErrorMessage name='end_time' class='error' />
			<VField
				v-model='form.end_time'
				rules='required|date|afterDate:begin_time'
				type='date'
				name='end_time'
				id='end_time'
				label='End date'
				aria-label='End date'
				class='form-control'
			/>
		</div>

		<div class='mb-2'>
			<label for='percentage' class='form-label'>Workload as a percentage</label>
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
		name: 'FormProjectNeed',

		props: {
			id: {
				type: String,
				default: undefined,
			},
			project_id: {
				type: String,
				required: true,
			},
			label: String,
			count_of_users: Number,
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
				const need = await this.$api.needs.save(this.form)
				if (need) this.$emit('success', need)
			},
		},
	}
</script>