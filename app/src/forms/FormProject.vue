<template>
	<VForm @submit='onSubmit' v-slot='{ errors }' class='vstack gap-2'>

		<div>
			<label for='name' class='form-label'>Project name</label>
			<VField
				v-model='form.name'
				rules='required'
				type='text'
				id='name'
				name='name'
				label='Project name'
				aria-label='Project name'
				class='form-control'
				:class='{ "is-invalid": errors.name }'
			/>
			<ErrorMessage name='name' class='invalid-feedback shake' />
		</div>

		<div>
			<label for='description' class='form-label'>Description</label>
			<VField
				v-model='form.description'
				type='text'
				id='description'
				name='description'
				label='Project description'
				aria-label='Project description'
				class='form-control'
			/>
		</div>

		<div class='form-check mb-0 py-2'>
			<label for='is_hidden' class='form-label mb-0'>Hidden</label>
			<VField
				v-model='form.is_hidden'
				:value='true'
				:unchecked-value='false'
				type='checkbox'
				id='is_hidden'
				name='is_hidden'
				class='form-check-input'
			/>
		</div>

		<div>
			<button type='submit' class='btn btn-primary gradient float-end'>Submit</button>
		</div>
	</VForm>
</template>

<script>
	export default {
		name: 'FormProject',

		props: {
			id: {
				type: String,
				default: undefined,
			},
			description: String,
			name: String,
			is_hidden: {
				type: Boolean,
				default: false,
			},
		},

		data() {
			return {
				form: { ...this.$props },
			}
		},

		methods: {
			async onSubmit() {
				const project = await this.$api.projects.save(this.form)
				if (project) this.$emit('success', project)
			},
		},
	}
</script>
