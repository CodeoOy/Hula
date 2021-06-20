<template>
	<form v-on:submit="addExistingSkill">
		<select class="form-select" id="AddExistingSkill" aria-label="Example select with button addon" v-model="queryData.skill_id">
			<option v-for="avskill in availableSkills" :key="avskill" :value="avskill.id">
				{{ avskill.label }}
			</option>
		</select>
		<select v-if="chosenSkill.skillscope_id" class="form-select" id="AddExistingSkill" aria-label="Example select with button addon" v-model="queryData.skillscopelevel_id">
			<option v-for="lvl in filterLevels" :key="lvl" :value="lvl.id">
				{{ lvl.label }}
			</option>
		</select>
		<input type="number" aria-label="Years" class="form-control" v-model.number="queryData.years">
		<button class="btn btn-outline-secondary" type="button">Button</button>
		<button type="submit" class="btn btn-gradient mb-1">Submit</button>
	</form>
</template>

<script>
export default {
	name: 'UserSkill',
	data() {
		return {
			user: {},
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
	methods: {
		addExistingSkill() {
			fetch(`/api/userskills/${this.user.id}`, {
				method: 'POST',
				headers: {"Content-Type": "application/json"},
				credentials: 'include',
				body: JSON.stringify(this.queryData)
			})
			.then(() => {
				this.$store.commit('setUser', this.user.id)
			})
			.catch((errors) => {
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
				this.skillLevels = response;
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