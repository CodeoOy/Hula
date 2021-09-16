<template>
	<VForm @submit='onSubmit' v-slot='{ errors }' class='vstack gap-2'>

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
				placeholder='Techs' 
				class='form-control'
				:class='{ "is-invalid": errors.label }'
			/>
			<ErrorMessage name='label' class='invalid-feedback shake' />
		</div>

		<div v-if='categories.length'>
			<label for='parent_id' class='form-label'>Parent category <span class='fw-light text-muted'>(optional)</span></label>
			<VField
				v-model='form.parent_id'
				as='select'
				name='parent_id'
				id='parent_id'
				label='Parent'
				aria-label='Parent'
				class='form-select'
			>
				<option :value='null'>No parent</option>
				<option v-for='category in categories' :key='category.id' :value='category.id'>
					{{ category.label }}
				</option>
			</VField>
		</div>

		<div class='mt-label'>
			<button type='submit' class='btn btn-primary gradient float-end'>Submit</button>
		</div>
	</VForm>   
</template>

<script>
	export default {
		name: 'FormSkillCategory',

		props: {
			id: {
				type: String,
				default: undefined,
			},
			parent_id: {
				type: String,
				default: undefined,
			},
			label: String,
		},

		data() {
			return {
				form: { ...this.$props },
				categories: {},
			}
		},

		async mounted() {
			this.categories = await this.$api.skills.categories.get()
		},

		methods: {
			async onSubmit() {
				const category = await this.$api.skills.categories.save(this.form)
				if (category) this.$emit('success', category)
			},
		},
	}
</script>