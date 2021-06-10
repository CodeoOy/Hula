<template>
	<form v-on:submit="UpdateProject">
		<div class="mb-2">
			<p v-if="errorsPresent" class="error">Please fill out label!</p>
			<div class="mb-2">
				<label class="form-label">Project name</label>
				<input class="form-control" type="text" placeholder="Project name" name="label" v-model="chosenproject.name" />
			</div>
			<button v-if="!('id' in chosenproject)" v-on:click="createProject" class="btn btn-gradient" type="button">Save</button>
		</div>
		<div v-if="'id' in chosenproject">
			<h3>{{ chosenproject.name }} needs</h3>
			<div v-for="need in chosenproject.needs" :key="need.id">
				<a href="#" v-on:click.prevent="this.chosenneed.id = need.id">{{ need.count_of_users}} from {{ need.begin_time }} to {{ need.end_time }} at percentage: {{ need.percentage}}</a>
				<table class="table table-dark table-striped text-light" v-if="'id' in chosenneed">
					<thead>
						<tr>
							<th scope="col">Skill</th>
							<th scope="col">Min level</th>
							<th scope="col">Min years</th>
							<th scope="col">Max years</th>
							<th scope="col">Actions</th>
						</tr>
					</thead>
					<tbody>
						<tr v-for="skill in need.skills" :key="skill.id">
							<td>{{ skill.id }}</td>
							<td>{{ skill.level }}</td>
							<td>{{ skill.min_years }}</td>
							<td>{{ skill.max_years }}</td>
							<td>Edit - Delete</td>
						</tr>
					</tbody>
				</table>
			</div>
		</div>
		<div v-if="'id' in chosenproject && !('id' in chosenneed)">
			<hr />
			<h3>New need</h3>
			<label class="form-label">How many pros for this need?</label>
			<input type="number" aria-label="Number of pros" class="form-control mb-2" v-model.number="querydata_need.count_of_users">
			<label class="form-label">When does this need start?</label>
			<input type="date" aria-label="Date start" class="form-control mb-2" v-model="querydata_need.begin_time">
			{{ querydata_need.begin_time }}
			<label class="form-label">When does this need end?</label>
			<input type="date" aria-label="Date end" class="form-control mb-2" v-model="querydata_need.end_time">
			<label class="form-label">At what percentage?</label>
			<input type="number" aria-label="Percentage" class="form-control mb-2" v-model.number="querydata_need.percentage">
			<button v-on:click="createProjectNeed" class="btn btn-gradient" type="button">Save need</button>
		</div>
		<div v-if="'id' in chosenneed">
			<hr />
			<h3>Add skill to this need</h3>
			<label class="form-label">Skill</label>
			<select class="form-select mb-2" id="AddExistingSkill" aria-label="Which skill" v-model="querydata_needskill.skill_id">
				<option v-for="avskill in available_skills" :key="avskill" :value="avskill.id">
					{{ avskill.label }}
				</option>
			</select>
			<label class="form-label">Minimum level</label>
			<select class="form-select mb-2" id="AddExistingSkill" aria-label="Which level" v-model="querydata_needskill.skillscopelevel_id">
				<option v-for="levelres in filterLevels" :key="levelres" :value="levelres.id">
					{{ levelres.label }}
				</option>
			</select>
			<button v-on:click="createProjectNeedSkill" class="btn btn-gradient" type="button">Save skill to need</button>
		</div>
		<button v-if="'id' in chosenproject" type="submit" class="btn btn-gradient mb-1">Save Project</button>
	</form> 
</template>

<script>
export default {
	name: 'FormProject',
	data() {
		return {
			querydata_need: {
				id: '06ba4809-f20b-4687-945b-e033a6751fca',
				project_id: '',
				count_of_users: Number,
				begin_time: "",
				end_time: "",
				percentage: Number,
				updated_by: this.$store.state.loggeduser.email
			},
			querydata_needskill: {
				id: '06ba4809-f20b-4687-945b-e033a6751fca',
				projectneed_id: '',
				skill_id: '',
				skillscopelevel_id: '',
				min_years: 1,
				max_years: 10,
				updated_by: this.$store.state.loggeduser.email
			},
			chosenskill: {
				type: Object,
				default() {
					return { 
						id: '',
						skillscope_id: '',
					}
				}
			},
			chosenneed: {
				type: Object,
				default() {
					return { 
						id: '',
					}
				}
			},
			available_skills: {},
			skill_levels: {},
			errorsPresent: false,
		}
	},
	props: {
		chosenproject: {
			type: Object,
			default() {
				return { id: '' }
			}
		},
	},
	methods: {
		createProject: function() {
			fetch('/api/projects', {
				method: 'POST',
				headers: {"Content-Type": "application/json"},
				credentials: 'include',
				body: JSON.stringify(this.chosenproject)
			})
			.then((response) => response.json())
			.then((response) => {
				this.chosenproject.id = response.id
				this.querydata_need.project_id = response.id
			})
		},
		updateProject: function() {
			fetch(`/api/projects/${this.chosenproject.id}`, {
				method: 'PUT',
				headers: {"Content-Type": "application/json"},
				credentials: 'include',
				body: JSON.stringify(this.chosenproject)
			})
			.then((response) => response.json())
			.then((response) => {
				this.chosenproject.id = response.id
				this.querydata_need.project_id = response.id
			})
		},
		createProjectNeed: function() {
			this.querydata_need.begin_time = `${this.querydata_need.begin_time}T00:00:00`
			this.querydata_need.end_time = `${this.querydata_need.end_time}T00:00:00`
			fetch('/api/projectneeds', {
				method: 'POST',
				headers: {"Content-Type": "application/json"},
				credentials: 'include',
				body: JSON.stringify(this.querydata_need)
			})
			.then((response) => response.json())
			.then(response => { 
				this.querydata_need = response;
				this.querydata_needskill.projectneed_id = response.id
			}) 
		},
		createProjectNeedSkill: function() {
			fetch('/api/projectskills', {
				method: 'POST',
				headers: {"Content-Type": "application/json"},
				credentials: 'include',
				body: JSON.stringify(this.querydata_needskill)
			})
			.then((response) => response.json())
			.then(response => { 
				this.chosenskill.id = response.id;
			}) 
		},
		getAllSkills: function() {
			fetch('/api/skills', {method: 'GET'})
			.then((response) => response.json())
			.then(response => { 
				this.available_skills = response;
			})    
			.catch((errors) => {
				//console.log(errors); // This gives unexpected end of json
			})
		},
		getAllLevels: function() {
			fetch('/api/skills/levels', {method: 'GET'})
			.then((response) => response.json())
			.then(response => { 
				this.skill_levels = response
			})    
			.catch((errors) => {
				//console.log(errors); // This gives unexpected end of json
			})
		},
		getSkillScope: function(needle) {
			var scope = this.available_skills.find(x => x.id == needle).skillscope_id;
			this.chosenskill.skillscope_id = scope;
		}
	},
	watch: {
		'chosenneed.id': function(newID, oldID) {
			this.querydata_needskill.projectneed_id = newID
		},
		'querydata_needskill.skill_id': function(newID, oldID) {
			this.getSkillScope(newID)
		},
		'chosenproject.id': function(newID, oldID) {
			console.log("Project id changed from " + oldID + " to " + newID)
			if (typeof newID !== 'undefined' && newID.length) {
				this.$store.commit('setChosenProject', newID)
				this.querydata_need.project_id = newID 
			} else {
				this.$store.commit('resetChosenProject')
			}
			this.chosenneed = {}
			this.chosenskill.id = ''
		}
	},
	computed: {
		filterLevels: function() {
			if ('skillscope_id' in this.chosenskill) {
				return this.skill_levels.filter(levelres => levelres.skillscope_id == this.chosenskill.skillscope_id)
			}
		},
		filterSkills: function(need) {
			//console.log("Skills:")
			//console.log(this.projectneedskills)
			return this.projectneedskills.filter(skill => skill.projectneed_id == need)
		}
	},
	mounted() {
		console.log("Mounted. chosen project:")
		console.log(this.chosenproject)
		this.getAllSkills()
		this.getAllLevels()
	}
};
</script>