<template>
	<v-form v-on:submit="addExistingSkill">
		<div class="mb-2">
			<label class="form-label">Skill</label>
			<error-message name="skill" class="error"></error-message>
			<v-field
				v-model="queryData.skill_id"
				:rules="isRequired"
				as="select"
				name="skill"
				class="form-select"
				id="AddExistingSkill"
				aria-label="Example select with button addon" 
			>
				<option v-for="avskill in availableSkills" :key="avskill" :value="avskill.id">
					{{ avskill.label }}
				</option>
			</v-field>
		</div>
		<div class="mb-2" v-if="chosenSkill.skillscope_id">
			<label class="form-label">Level of skill</label>
			<error-message name="level" class="error"></error-message>
			<v-field
				v-model="queryData.skillscopelevel_id"
				:rules="isRequired"
				as="select"
				name="level"
				class="form-select"
				id="AddExistingSkill"
				aria-label="Example select with button addon"
			>
				<option v-for="lvl in filterLevels" :key="lvl" :value="lvl.id">
					{{ lvl.label }}
				</option>
			</v-field>
		</div>
		<div class="mb-2" v-if="chosenSkill.skillscope_id">
			<label class="form-label">Years</label>
			<error-message name="years" class="error"></error-message>
			<v-field
				v-model.number="queryData.years"
				:rules="isRequired"
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
			chosenSkill: {
				id: '',
				skillscope_id: '',
			},
			queryData: {
				id: '83d7a553-2e53-47cc-8c16-a8a10c0cadd0', // TODO: This is here only to satisfy UserSkill struct. Remove somehow.
				user_id: this.$store.state.loggeduser.id,
				skill_id: '',
				skillscopelevel_id: '',
				years: Number,
				updated_by: this.$store.state.loggeduser.email,
			},
			availableSkills: {},
			skillLevels: [],
		}
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
		addExistingSkill() {
			fetch(`/api/userskills/${this.$store.state.loggeduser.id}`, {
				method: 'POST',
				headers: {"Content-Type": "application/json"},
				credentials: 'include',
				body: JSON.stringify(this.queryData)
			})
			.then(() => {
				this.$store.dispatch('setUser', this.$store.state.loggeduser.id)
				this.$emit('formSent')
			})
			.catch((errors) => {
				this.$store.commit('errorHandling', errors)
			})
		},
		getAllSkills() {
			fetch('/api/skills', {method: 'GET'})
			.then((response) => response.json())
			.then(response => { 
				this.availableSkills = response;
			})    
			.catch((errors) => {
				this.$store.commit('errorHandling', errors)
			})
		},
		getAllLevels() {
			fetch('/api/skills/levels', {method: 'GET'})
			.then((response) => response.json())
			.then(response => { 
				this.skillLevels = response;
			})    
			.catch((errors) => {
				this.$store.commit('errorHandling', errors)
			})
		},
		getSkillScope(needle) {
			var scope = this.availableSkills.find(x => x.id == needle).skillscope_id;
			this.chosenSkill.skillscope_id = scope;
		}
	},
	watch: {
		'queryData.skill_id'(newID) {
			this.getSkillScope(newID)
		}
	},
	computed: {
		filterLevels() {
			return this.skillLevels.filter(lvl => lvl.skillscope_id == this.chosenSkill.skillscope_id)
		}
	},
	mounted() {
		this.user = this.$store.state.loggeduser
		this.getAllSkills()
		this.getAllLevels()
	}
};
</script>