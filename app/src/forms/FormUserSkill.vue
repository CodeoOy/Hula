<template>
	<v-form v-on:submit="saveUserSkill" v-if="availableSkills.length && skillLevels.length">
		<div class="mb-2">
			<label class="form-label">Skill</label>
			<error-message name="skill" class="error"></error-message>
			<v-field
				v-model="formData.skill_id"
				:rules="isRequired"
				as="select"
				name="skill"
				class="form-select"
				aria-label="Example select with button addon" 
			>
				<option :value="null" disabled selected>Pick a skill</option>
				<option v-for="skill in availableSkills" :key="skill" :value="skill.id">
					{{ skill.label }}
				</option>
			</v-field>
		</div>
		<div class="mb-2" v-if="'skillscopelevel_id' in formData">
			<label class="form-label">Level of skill</label>
			<error-message name="level" class="error"></error-message>
			<v-field
				v-model="formData.skillscopelevel_id"
				as="select"
				name="level"
				class="form-select"
				aria-label="Example select with button addon"
			>
				<option :value="null" disabled selected>No level</option>
				<option v-for="lvl in filterLevels" :key="lvl" :value="lvl.id">
					{{ lvl.label }}
				</option>
			</v-field>
		</div>
		<div class="mb-2">
			<label class="form-label">Years</label>
			<error-message name="years" class="error"></error-message>
			<v-field
				v-model.number="formData.years"
				as="input"
				type="number"
				name="years"
				class="form-control"
				aria-label="Years"
			></v-field>
		</div>
		<button type="submit" class="btn btn-gradient mb-1">Submit</button>
	</v-form>
</template>

<script>
import { Field, Form, ErrorMessage } from 'vee-validate';
export default {
	name: 'UserSkill',
	data() {
		return {
			formData: {
				id: this.chosenSkill.id || undefined,
				user_id: this.userID,
				skill_id: this.chosenSkill.skill_id || '',
				skillscopelevel_id: this.chosenSkill.skillscopelevel_id || '',
				years: this.chosenSkill.years || '',
				updated_by: this.$store.state.loggeduser.email,
			},
			availableSkills: {},
			skillLevels: [],
		}
	},
	props: {
		url: '',
		method: '',
		userID: '',
		chosenSkill: {},
		userSkills: {},
	},
	components: {
		'VForm': Form,
		'VField': Field,
		ErrorMessage
	},
	methods: {
		isRequired(value) {
			return value ? true : 'This field is required';
		},
		async saveUserSkill() {
			for (let prop in this.formData) {
				if (this.formData[prop] === '') {
					delete this.formData[prop]
				}
			}
			const skill = await this.$api.users.skills.save(this.formData)
			if (skill) this.$emit('formSent')
		},
		getSkillScope(needle) {
			if (needle) {
				var scope = this.availableSkills.find(x => x.id == needle).skillscope_id;
				this.chosenSkill.skillscope_id = scope;
			}
		},
	},
	watch: {
		'formData.skill_id'(newID) {
			this.getSkillScope(newID)
		}
	},
	computed: {
		filterLevels() {
			if (this.chosenSkill.skillscope_id) {
				return this.skillLevels.filter(lvl => lvl.skillscope_id == this.chosenSkill.skillscope_id)
			}
			return []
		}
	},
	async mounted() {
		this.user = this.userID
		const [
			skills,
			levels,
		] = await Promise.all([
			this.$api.skills.get(),
			this.$api.skills.levels.get(),
		])
		this.availableSkills = skills
		this.skillLevels = levels
	}
};
</script>