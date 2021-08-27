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
				placeholder='Techs' 
				class='form-control'
			/>
		</div>

		<div class='mb-2' v-if='categories.length'>
			<label for='parent_id' class='form-label'>Parent category (optional)</label>
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

		<button type='submit' class='btn btn-gradient'>Submit</button>
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