<template>
	<VForm @submit='onSubmit' v-slot='{ errors }'>

		<div class='mb-2'>
			<label for='label' class='form-label'>Label</label>
			<VField
				v-model='form.label'
				rules='required'
				type='text'
				name='label'
				id='label'
				label='Label'
				aria-label='Label'
				class='form-control'
				:class='{ "is-invalid": errors.label }'
			/>
			<ErrorMessage name='label' class='invalid-feedback shake' />
		</div>

		<div class='mb-2'>
			<label for='count_of_users' class='form-label'>Number of positions</label>
			<VField
				v-model.number='form.count_of_users'
				rules='required'
				type='number'
				name='count_of_users'
				id='count_of_users'
				label='Number of positions'
				aria-label='Number of positions'
				class='form-control'
				:class='{ "is-invalid": errors.count_of_users }'
			/>
			<ErrorMessage name='count_of_users' class='invalid-feedback shake' />
		</div>

		<div class='mb-2'>
			<label for='begin_time' class='form-label'>Start date</label>
			{{ begin_time }}<br />
			<VField
				v-model='form.begin_time'
				rules='required|date'
				type='date'
				name='begin_time'
				id='begin_time'
				label='Start date' 
				aria-label='Start date' 
				class='form-control'
				:class='{ "is-invalid": errors.begin_time }'
			/>
			<ErrorMessage name='begin_time' class='invalid-feedback shake' />
		</div>

		<div class='mb-2'>
			<label for='end_time' class='form-label'>End date</label>
			{{ end_time }}<br />
			<VField
				v-model='form.end_time'
				rules='required|date|afterDate:begin_time'
				type='date'
				name='end_time'
				id='end_time'
				label='End date'
				aria-label='End date'
				class='form-control'
				:class='{ "is-invalid": errors.end_time }'
			/>
			<ErrorMessage name='end_time' class='invalid-feedback shake' />
		</div>

		<div class='mb-2'>
			<label for='percentage' class='form-label'>Workload as a percentage</label>
			<VField
				v-model.number='form.percentage'
				rules='required'
				type='number'
				name='percentage'
				id='percentage'
				label='Percentage'
				aria-label='Percentage'
				class='form-control'
				:class='{ "is-invalid": errors.percentage }'
			/>
			<ErrorMessage name='percentage' class='invalid-feedback shake' />
		</div>

		<button type='submit' class='btn btn-primary gradient mb-1'>Save</button>
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