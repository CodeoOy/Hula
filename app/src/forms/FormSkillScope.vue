<template>
	<VForm v-on:submit='onSubmit'>

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
				placeholder='One to ten' 
				class='form-control'
			/>
		</div>

		<button type='submit' class='btn btn-gradient mb-1'>Submit</button>
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