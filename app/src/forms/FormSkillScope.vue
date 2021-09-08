<template>
	<VForm @submit='onSubmit' v-slot='{ errors }'>

		<div class='mb-2'>
			<label for='label' class='form-label'>Name</label>
			<VField
				v-model='form.label'
				rules='required'
				type='text'
				id='label'
				name='label'
				label='Name'
				aria-label='Name'
				placeholder='One to ten' 
				class='form-control'
				:class='{ "is-invalid": errors.label }'
			/>
			<ErrorMessage name='label' class='invalid-feedback shake' />
		</div>

		<button type='submit' class='btn btn-primary gradient mb-1'>Submit</button>
	</VForm>  
</template>

<script>
	export default {
		name: 'FormSkillScope',

		props: {
			id: {
				type: String,
				default: undefined,
			},
			label: String,
		},	

		data() {
			return {
				form: { ...this.$props },
			}
		},

		methods: {
			async onSubmit() {
				const scope = await this.$api.skills.scopes.save(this.form)
				if (scope) this.$emit('success', scope)
			}
		},
	}
</script>