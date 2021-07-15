<template>
	<v-form v-on:submit="createProjectNeedSkill">
		<h2>Add skill to this need</h2>
		<div class="mb-2">
			<label class="form-label">Skill</label>
			<error-message name="name" class="error"></error-message>
			<v-field
				v-model="queryDataNeedSkill.skill_id"
				:rules="isRequired"
				as="select"
				name="skill"
				class="form-select"
				id="AddExistingSkill"
				aria-label="Pick skill"
			>
				<option v-for="avskill in filterSkills" :key="avskill" :value="avskill.id">
					{{ avskill.label }}
				</option>
			</v-field>
		</div>
		<div class="mb-2">
			<label class="form-label">Minimum level</label>
			<error-message name="name" class="error"></error-message>
			<v-field
				v-model="queryDataNeedSkill.skillscopelevel_id"
				:rules="isRequired"
				as="select"
				name="skillscope"
				class="form-select"
				id="AddLevel"
				aria-label="Pick minimum level"
			>
				<option v-for="lvl in filterLevels" :key="lvl" :value="lvl.id">
					{{ lvl.label }}
				</option>
			</v-field>
		</div>
		<button type="submit" class="btn btn-gradient mb-1">Save</button>
	</v-form>
</template>

<script>
import { Field, Form, ErrorMessage } from 'vee-validate';
export default {
	name: 'ProjectNeedSkill',
	data() {
		return {
			queryDataNeedSkill: {
				projectneed_id: this.chosenNeed.id,
				skill_id: '',
				skillscopelevel_id: '',
				min_years: 1,
				max_years: 10
			},
			chosenSkill: {
				type: Object,
				default() {
					return { 
						id: '',
						skillscope_id: '',
					}
				}
			},
			availableSkills: {},
			skillLevels: {},
			errors: []
		}
	},
	components: {
		'VForm': Form,
		'VField': Field,
		ErrorMessage
	},
	props: {
		chosenNeed: {
			type: Object,
			default() {
				return { 
					id: '',
				}
			}
		},
	},
	methods: {
		isRequired(value) {
			return value ? true : 'This field is required';
		},
		createProjectNeedSkill() {
			fetch('/api/projectskills', {
				method: 'POST',
				headers: {"Content-Type": "application/json"},
				credentials: 'include',
				body: JSON.stringify(this.queryDataNeedSkill)
			})
			.then(() => {
				this.$emit('formSent')
			})
			.catch((errors) => {
				this.errors.push('skill-error')
				this.$store.commit('errorHandling', errors)
			})
		},
		getAllSkills() {
			fetch('/api/skills', {method: 'GET'})
			.then(response => { 
				return (response.status >= 200 && response.status <= 299) ? response.json() : this.$store.commit('errorHandling', response)
			})
			.then(response => { 
				this.availableSkills = response;
			})    
			.catch((errors) => {
				this.$store.commit('errorHandling', errors)
			})
		},
		getAllLevels() {
			fetch('/api/skills/levels', {method: 'GET'})
			.then(response => { 
				return (response.status >= 200 && response.status <= 299) ? response.json() : this.$store.commit('errorHandling', response)
			})
			.then(response => { 
				this.skillLevels = response
			})    
			.catch((errors) => {
				this.$store.commit('errorHandling', errors)
			})
		},
		getSkillScope(needle) {
			var scope = this.availableSkills.find(x => x.id == needle).skillscope_id;
			this.chosenSkill.skillscope_id = scope;
		},
	},
	watch: {
		'queryDataNeedSkill.skill_id'(newID) {
			this.getSkillScope(newID)
		},
		'chosenNeed.id'(newID) {
			this.queryDataNeedSkill.projectneed_id = newID
		},
	},
	computed: {
		filterLevels() {
			if ('skillscope_id' in this.chosenSkill) {
				return this.skillLevels.filter(lvl => lvl.skillscope_id == this.chosenSkill.skillscope_id)
			}
		},
		// get skills from availableSkills that are not listed in chosenNeed.skills
		filterSkills() {
			if (this.availableSkills.length) {
				return this.availableSkills.filter(x => !this.chosenNeed.skills.find(y => y.skill_id == x.id));
			}
		}
	},
	mounted() {
		this.getAllSkills()
		this.getAllLevels()
	}
};
</script>