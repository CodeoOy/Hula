<template>
	<VForm @submit="onSubmit" v-slot='{ errors }' class='vstack gap-2'>

		<div>
			<label for='label' class='form-label'>Name</label>
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
				:class='{ "is-invalid": errors.label }'
			/>
			<ErrorMessage name='label' class='invalid-feedback shake' />
		</div>

		<div>
			<label for='percentage' class='form-label'>Percentage</label>
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

		<div class='mt-label'>
			<button type='submit' :disabled='sending' class='btn btn-primary gradient float-end'>{{ submitLabel }}</button>
		</div>
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
				sending: false,
				form: { ...this.$props },
			}
		},

		computed: {
			submitLabel() {
				return this.sending ? 'Saving' : 'Save'
			},
		},

		methods: {
			async onSubmit() {
				this.sending = true

				const level = await this.$api.skills.levels.save(this.form)
				if (level) this.$emit('success', level)

				this.sending = false
			},
		},
	}
</script>
