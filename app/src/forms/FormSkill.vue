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
				class='form-control'
				:class='{ "is-invalid": errors.label }'
			/>
			<ErrorMessage name='label' class='invalid-feedback shake' />
		</div>

		<div class='mb-2' v-if='form.id'>
			<label for='skillcategory_id' class='form-label'>Category</label>
			<VField
				v-model='form.skillcategory_id'
				rules='required'
				as='select'
				name='skillcategory_id'
				id='skillcategory_id'
				label='Category'
				aria-label='Category'
				class='form-select'
				:class='{ "is-invalid": errors.skillcategory_id }'
			>
				<option v-for="category in categories" :key="category.id" :value="category.id">
					{{ category.label }}
				</option>
			</VField>
			<ErrorMessage name='skillcategory_id' class='invalid-feedback shake' />
		</div>

		<div class='mb-2' v-if='!form.id'>
			<label for='skillscope_id' class='form-label'>Scope</label>
			<VField
				v-model='form.skillscope_id'
				rules='required'
				as='select'
				name='skillscope_id'
				id='skillscope_id'
				label='Scope'
				aria-label='Scope'
				class='form-select'
				:class='{ "is-invalid": errors.skillscope_id }'
			>
				<option v-for="scope in scopes" :key="scope.id" :value="scope.id">
					{{ scope.label }}
				</option>
			</VField>
			<ErrorMessage name='skillscope_id' class='invalid-feedback shake' />
		</div>

		<button type='submit' class='btn btn-primary gradient mb-1'>Submit</button>
	</VForm> 
</template>

<script>
	export default {
		name: 'FormSkill',

		props: {
			id: {
				type: String,
				default: undefined,
			},
			skillcategory_id: {
				type: String,
				required: true,
			},
			skillscope_id: String,
			label: String
		},

		computed: {
			categories() {
				return this.$store.state.skillCategories
			},

			scopes() {
				return this.$store.state.skillScopes
			},
		},

		data() {
			return {
				form: { ...this.$props },
			}
		},
		
		mounted() {
			if (!this.$store.state.skillCategories.length) this.$store.dispatch('getSkillCategories')
			if (!this.$store.state.skillScopes.length) this.$store.dispatch('getSkillScopes')
		},

		methods: {
			async onSubmit() {
				const skill = await this.$api.skills.save(this.form)
				if (skill) this.$emit('success', skill)
			},
		},
	}
</script>