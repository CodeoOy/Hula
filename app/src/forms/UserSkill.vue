<template>
	<form action="#" @submit.prevent="addExistingSkill">
		<select class="form-select" id="AddExistingSkill" aria-label="Example select with button addon" v-model="querydata.skill_id">
			<option v-for="avskill in available_skills" :key="avskill" :value="avskill.id">
				{{ avskill.label }}
			</option>
		</select>
		<select v-if="chosenskill.skillscope_id" class="form-select" id="AddExistingSkill" aria-label="Example select with button addon" v-model="querydata.skillscopelevel_id">
			<option v-for="levelres in filterLevels" :key="levelres" :value="levelres.id">
				{{ levelres.label }}
			</option>
		</select>
		<input type="number" aria-label="Years" class="form-control" v-model.number="querydata.years">
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
			editing_info: false,
			chosenskill: {
				id: '',
				skillscope_id: '',
			},
			querydata: {
				id: '83d7a553-2e53-47cc-8c16-a8a10c0cadd0', // TODO: This is here only to satisfy UserSkill struct. Remove somehow.
				user_id: this.$store.state.loggeduser.id,
				skill_id: '',
				skillscopelevel_id: '',
				years: Number,
				updated_by: this.$store.state.loggeduser.email,
			},
			available_skills: {},
			skill_levels: [],
		}
	},
	methods: {
		addExistingSkill: function() {
			console.log(this.querydata)
			fetch(`/api/userskills/${this.user.id}`, {
				method: 'POST',
				headers: {"Content-Type": "application/json"},
				credentials: 'include',
				body: JSON.stringify(this.querydata)
			})
			.then(() => {
				this.$store.commit('setUser', this.user.id)
			})
		},
		getAllSkills: function() {
			fetch('/api/skills', {method: 'GET'})
			.then((response) => response.json())
			.then(response => { 
				this.available_skills = response;
			})    
			.catch((errors) => {
				console.log(errors); // This gives unexpected end of json
			})
		},
		getAllLevels: function() {
			fetch('/api/skills/levels', {method: 'GET'})
			.then((response) => response.json())
			.then(response => { 
				this.skill_levels = response;
			})    
			.catch((errors) => {
				console.log(errors); // This gives unexpected end of json
			})
		},
		getSkillScope: function(needle) {
			console.log("Input happened")
			var scope = this.available_skills.find(x => x.id == needle).skillscope_id;
			this.chosenskill.skillscope_id = scope;
			console.log(this.querydata.id)
		}
	},
	watch: {
		'querydata.skill_id': function(newID, oldID) {
			console.log("Title changed from " + oldID + " to " + newID)
			this.getSkillScope(newID)
		}
	},
	computed: {
		filterLevels: function() {
			console.log(this.chosenskill.skillscope_id)
			return this.skill_levels.filter(levelres => levelres.skillscope_id == this.chosenskill.skillscope_id)
		}
	},
	mounted() {
		this.user = this.$store.state.loggeduser
		this.getAllSkills()
		this.getAllLevels()
	}
};
</script>