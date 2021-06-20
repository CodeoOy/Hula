<template>
	<form v-on:submit="createProjectNeedSkill">
		<h3>Add skill to this need</h3>
		<p v-if="errors.length && errors.includes('skill-error')" class="error">Error in adding skill. Maybe it's already added?</p>
		<label class="form-label">Skill</label>
		<select class="form-select mb-2" id="AddExistingSkill" aria-label="Which skill" v-model="queryDataNeedSkill.skill_id">
			<option v-for="avskill in availableSkills" :key="avskill" :value="avskill.id">
				{{ avskill.label }}
			</option>
		</select>
		<label class="form-label">Minimum level</label>
		<select class="form-select mb-2" id="AddExistingSkill" aria-label="Which level" v-model="queryDataNeedSkill.skillscopelevel_id">
			<option v-for="lvl in filterLevels" :key="lvl" :value="lvl.id">
				{{ lvl.label }}
			</option>
		</select>
		<button type="submit" class="btn btn-gradient mb-1">Save</button>
	</form>
</template>

<script>
export default {
	name: 'ProjectNeedSkill',
	data() {
		return {
			queryDataNeedSkill: {
				id: '06ba4809-f20b-4687-945b-e033a6751fca',
				projectneed_id: this.chosenNeed.id,
				skill_id: '',
				skillscopelevel_id: '',
				min_years: 1,
				max_years: 10,
				updated_by: this.$store.state.loggeduser.email
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
		createProjectNeedSkill() {
			fetch('/api/projectskills', {
				method: 'POST',
				headers: {"Content-Type": "application/json"},
				credentials: 'include',
				body: JSON.stringify(this.queryDataNeedSkill)
			})
			.catch((errors) => {
				this.errors.push('skill-error')
				console.log(errors);
			})
		},
		getAllSkills() {
			fetch('/api/skills', {method: 'GET'})
			.then((response) => response.json())
			.then(response => { 
				this.availableSkills = response;
			})    
			.catch((errors) => {
				console.log(errors);
			})
		},
		getAllLevels() {
			fetch('/api/skills/levels', {method: 'GET'})
			.then((response) => response.json())
			.then(response => { 
				this.skillLevels = response
			})    
			.catch((errors) => {
				console.log(errors);
			})
		},
		getSkillScope(needle) {
			var scope = this.availableSkills.find(x => x.id == needle).skillscope_id;
			this.chosenSkill.skillscope_id = scope;
		}
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
		}
	},
	mounted() {
		this.getAllSkills()
		this.getAllLevels()
	}
};
</script>