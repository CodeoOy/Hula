<template>
	<VForm @submit='onSubmit' v-slot='{ errors }'>

		<div class='mb-2' v-if='!id'>
			<label for='skill' class='form-label'>Skill</label>
			<VField
				v-model='form.skill_id'
				rules='required'
				as='select'
				name='skill'
				id='skill'
				label='Skill'
				aria-label='Skill'
				class='form-select'
				:class='{ "is-invalid": errors.skill }'
			>
				<option :value='null' disabled selected>Pick a skill</option>
				<option v-for='skill in filteredSkills' :key='skill.id' :value='skill.id'>
					{{ skill.label }}
				</option>
			</VField>
			<ErrorMessage name='skill' class='invalid-feedback shake' />
		</div>

		<div class='mb-2 form-check'>
			<label for='mandatory' class='form-label'>Mandatory requirement</label>
			<VField
				v-model.boolean='form.mandatory'
				:value='true'
				:unchecked-value='false'
				type='checkbox'
				name='mandatory'
				id='mandatory'
				aria-label='Skill'
				class="form-check-input"
			/>
		</div>

		<div class='mb-2'>
			<label for='skillscopelevel_id' class='form-label'>Minimum level</label>
			<VField
				v-model='form.skillscopelevel_id'
				:disabled='!form.skill_id'
				as='select'
				name='skillscopelevel_id'
				id='skillscopelevel_id'
				label='Minimum level'
				aria-label='Minimum level'
				class='form-select'
			>
				<option v-for='level in filteredLevels' :key='level.id' :value='level.id'>
					{{ level.label }}
				</option>
			</VField>
		</div>

		<div class='mb-2'>
			<label for='min_years' class='form-label'>Min years</label>
			<VField
				v-model.number='form.min_years'
				type='number'
				name='min_years'
				id='min_years'
				label='Min years'
				aria-label='Min years'
				class='form-control'
			/>
		</div>

		<div class='mb-2'>
			<label for='max_years' class='form-label'>Max years</label>
			<VField
				v-model.number='form.max_years'
				:rules='`min_value:${form.min_years}`'
				type='number'
				name='max_years'
				id='max_years'
				label='Max years'
				aria-label='Max years'
				class='form-control'
			/>
		</div>

		<button type='submit' class='btn btn-primary gradient mb-1'>Save</button>
	</VForm>
</template>

<script>
	export default {
		name: 'FormProjectNeedSkill',

		props: {
			id: {
				type: String,
				default: undefined,
			},
			projectneed_id: {
				type: String,
				required: true,
			},
			skill_id: String,
			skillscopelevel_id: String,
			min_years: Number,
			max_years: Number,
			mandatory: {
				type: Boolean,
				default: false,
			},
			disabledSkills: {
				type: Array,
				default: [],
			},
		},

		data() {
			const {
				disabledSkills,
				...props
			} = this.$props

			return {
				form: { ...props },
			}
		},

		computed: {
			skills() {
				return this.$store.state.skills
			},

			levels() {
				return this.$store.state.skillLevels
			},

			scopeId() {
				const skill = this.skills.find(skill => skill.id == this.form.skill_id)
				return skill ? skill.skillscope_id : null
			},

			filteredSkills() {
				return this.skills.filter(skill => !this.disabledSkills.includes(skill.id))
			},

			filteredLevels() {
				return this.levels.filter(level => level.skillscope_id == this.scopeId)
			},
		},

		mounted() {
			if (!this.$store.state.skills.length) this.$store.dispatch('getSkills')
			if (!this.$store.state.skillLevels.length) this.$store.dispatch('getSkillLevels')
		},

		methods: {
			async onSubmit() {
				// Remove empty strings from payload but preserve false booleans
				for (const prop in this.form) if (this.form[prop] === '') this.form[prop] = undefined
				const skill = await this.$api.needs.skills.save(this.form)
				if (skill) this.$emit('success', skill)
			},
		},
	}
</script>