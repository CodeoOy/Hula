<template>
	<VForm v-on:submit="onSubmit">

		<div class='mb-2'>
			<label for='label' class='form-label'>Name</label>
			<ErrorMessage name='label' class='error' />
			<VField
				v-model='form.label'
				rules='required'
				type='text'
				id='label'
				name='label'
				label='Name'
				aria-label='Name'
				placeholder='Rookie' 
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

		<button type="submit" class="btn btn-gradient mb-1">Submit</button>
	</VForm> 
</template>

<script>
	export default {
		name: 'FormSkillScopeLevel',

		props: {
			id: {
				type: String,
				default: undefined,
			},
			label: String,
			percentage: Number,
			skillscope_id: String,
			email: String,
		},

		data() {
			return {
				form: { ...this.$props },
			}
		},

		methods: {
			async onSubmit() {
				const level = await this.$api.skills.levels.save(this.form)
				if (level) this.$emit('success', level)
			},
		},
	}
</script>