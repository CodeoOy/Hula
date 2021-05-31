<template>
	<div>
		<form>
			<div v-if="!querydata_project.id.length" class="mb-2">
				<p v-if="errorsPresent" class="error">Please fill out label!</p>
				<div class="mb-2">
					<label class="form-label">Project name</label>
					<input class="form-control" type="text" placeholder="Languages" name="label" v-model="querydata_project.name" />
				</div>
				<button v-on:click="createProject" class="btn btn-gradient" type="button">Save</button>
			</div>
			{{ querydata_project.name }}
			<div v-if="projectneeds.length">
				<h3>Skills</h3>
				<table class="table table-dark table-striped text-light">
					<thead>
						<tr>
							<th scope="col">Skill</th>
							<th scope="col">Years</th>
							<th scope="col">Actions</th>
						</tr>
					</thead>
					<tbody>
						<tr v-for="skill in chosenskills" :key="skill.id">
							<td>{{ skill.skill_label }}</td>
							<td>{{ skill.years }}</td>
							<td>Edit - Delete</td>
						</tr>
					</tbody>
				</table>
			</div>
			<div v-if="querydata_project.id.length">
				<select class="form-select" id="AddExistingSkill" aria-label="Example select with button addon" v-model="querydata_skill.skill_id">
					<option v-for="avskill in available_skills" :key="avskill" :value="avskill.id">
						{{ avskill.label }}
					</option>
				</select>
				<select class="form-select" id="AddExistingSkill" aria-label="Example select with button addon" v-model="querydata_skill.skillscopelevel_id">
					<option v-for="levelres in filterLevels" :key="levelres" :value="levelres.id">
						{{ levelres.label }}
					</option>
				</select>
				<button v-on:click="createProjectNeed" class="btn btn-gradient" type="button">Add</button>
			</div>
			<div>

			</div>
			<button type="submit" class="btn btn-gradient mb-1">Save</button>
		</form> 
	</div>
</template>

<script>
export default {
	name: 'Project',
	data() {
		return {
			querydata_project: {
				id: '',
				name: ''
			},
			querydata_need: {
				id: '06ba4809-f20b-4687-945b-e033a6751fca',
				project_id: '',
				countofusers: 1,
				begin_time: "2012-01-01T00:00:00",
				end_time: "2012-01-01T00:00:00",
				percentage: 10,
				updated_by: this.$store.state.loggeduser.id
			},
			querydata_needskill: {
				id: '06ba4809-f20b-4687-945b-e033a6751fca',
				projectneed_id: '',
				skill_id: '',
				skillscopelevel_id: '',
				min_years: 1,
				max_years: 10,
				updated_by: this.$store.state.loggeduser.id
			},
			chosenskill: {
				id: '',
				skillscope_id: '',
			},
			projectneedskills: {},
			projectneeds: {},
			available_skills: {},
			skill_levels: [],
		}
	},
	methods: {
		createProject: function() {
			fetch('/api/projects', {
				method: 'POST',
				headers: {"Content-Type": "application/json"},
				credentials: 'include',
				body: JSON.stringify(this.querydata_project)
			})
			.then((response) => response.json())
			.then((response) => {
				console.log(response)
				this.querydata_project.id = response.id
				this.querydata_skill.project_id = response.id
			})
		},
		createProjectNeed: function() {
			fetch('/api/projectneeds', {
				method: 'POST',
				headers: {"Content-Type": "application/json"},
				credentials: 'include',
				body: JSON.stringify(this.querydata_skill)
			})
			this.getProjectNeeds()
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
		getProjectNeeds: function() {
			fetch(`/api/projectneeds/${this.querydata_project.id}`, {
				method: 'GET',
				headers: {"Content-Type": "application/json"},
				credentials: 'include'
			})
			.then((response) => response.json())
			.then((response) => {
				console.log(response)
				this.projectneeds = response
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
			var scope = this.available_skills.find(x => x.id == needle).skillscope_id;
			this.chosenskill.skillscope_id = scope;
		}
	},
	watch: {
		'querydata_skill.skill_id': function(newID, oldID) {
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
		this.getProjectNeeds()
		this.getAllSkills()
		this.getAllLevels()
	}
};
</script>