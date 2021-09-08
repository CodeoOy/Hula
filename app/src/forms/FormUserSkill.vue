<template>
	<VForm v-if='skills.length && levels.length' @submit='onSubmit' v-slot='{ errors }'>

		<div class='mb-2'>
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
				<option v-for='skill in skills' :key='skill.id' :value='skill.id'>
					{{ skill.label }}
				</option>
			</VField>
			<ErrorMessage name='skill' class='invalid-feedback shake' />
		</div>

		<div class='mb-2'>
			<label for='level' class='form-label'>Skill level</label>
			<VField
				v-model='form.skillscopelevel_id'
				rules='required'
				as='select'
				name='level'
				id='level'
				label='Skill level'
				aria-label='Skill level'
				class='form-select'
				:class='{ "is-invalid": errors.level }'
			>
				<option :value='null' disabled selected>No level</option>
				<option v-for='level in filteredLevels' :key='level.id' :value='level.id'>
					{{ level.label }}
				</option>
			</VField>
			<ErrorMessage name='level' class='invalid-feedback shake' />
		</div>

		<div class='mb-2'>
			<label for='years' class='form-label'>Years</label>
			<VField
				v-model.number='form.years'
				type='number'
				name='years'
				id='years'
				label='Years'
				aria-label='Years'
				class='form-control'
			/>
		</div>

		<button type='submit' class='btn btn-primary gradient mb-1'>Submit</button>
	</VForm>
</template>

<script>
	export default {
		name: 'FormUserSkill',

		props: {
			id: {
				type: String,
				default: undefined,
			},
			user_id: {
				type: String,
				required: true,
			},
			skill_id: String,
			skillscopelevel_id: String,
			years: Number,
		},

		data() {
			return {
				form: {
					...this.$props,
					updated_by: this.$store.state.loggeduser.email,
				},
				skills: {},
				levels: [],
			}
		},

		computed: {
			scopeId() {
				const skill = this.skills.find(skill => skill.id == this.form.skill_id)
				return skill ? skill.skillscope_id : null
			},

			filteredLevels() {
				return this.levels.filter(level => level.skillscope_id == this.scopeId)
			},
		},

		async mounted() {
			const [
				skills,
				levels,
			] = await Promise.all([
				this.$api.skills.get(),
				this.$api.skills.levels.get(),
			])
			this.skills = skills
			this.levels = levels
		},

		methods: {
			async onSubmit() {
				for (const prop in this.form) if (this.form[prop] == '') this.form[prop] = undefined
				const skill = await this.$api.users.skills.save(this.form)
				if (skill) this.$emit('success', skill)
			},
		},
	}
</script>