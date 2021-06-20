<template>
	<form v-on:submit="updateProject">
		<div class="mb-2">
			<p v-if="errors.length && errors.includes('project-error')" class="error">Please fill out label!</p>
			<div class="mb-2">
				<label class="form-label">Project name</label>
				<input class="form-control" type="text" placeholder="Project name" name="label" v-model="chosenProject.name" />
			</div>
			<button v-if="!('id' in chosenProject)" v-on:click="createProject" class="btn btn-gradient" type="button">Save</button>
		</div>
		<div v-if="'id' in chosenProject">
			<h3>{{ chosenProject.name }} needs</h3>
			<div class="accordion accordion-flush" id="needsAccordion">
				<div class="accordion-item bg-dark" v-for="need in chosenProject.needs" :key="need.id">
					<h4 class="accordion-header" :id="'heading' + need.id">
					<button v-on:click.prevent="this.chosenNeed.id = need.id" class="accordion-button collapsed" type="button" data-bs-toggle="collapse" :data-bs-target="'#acc-' + need.id" aria-expanded="true" :aria-controls="need.id">
						{{ need.count_of_users}} from {{ need.begin_time }} to {{ need.end_time }} at percentage: {{ need.percentage}}
					</button>
					</h4>
					<div :id="'acc-' + need.id" class="accordion-collapse collapse" :aria-labelledby="'heading' + need.id" data-bs-parent="#needsAccordion">
						<div class="accordion-body">
							<table class="table table-dark table-striped text-light">
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
							<div v-if="'id' in chosenNeed">
								<hr />
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
								<button v-on:click="createProjectNeedSkill" class="btn btn-gradient" type="button">Save skill to need</button>
							</div>
						</div>
					</div>
				</div>
			</div>
		</div>
		<div v-if="'id' in chosenProject && !('id' in chosenNeed)">
			<hr />
			<h3>New need</h3>
			<label class="form-label">How many pros for this need?</label>
			<input type="number" aria-label="Number of pros" class="form-control mb-2" v-model.number="queryDataNeed.count_of_users">
			<label class="form-label">When does this need start?</label>
			<input type="date" aria-label="Date start" class="form-control mb-2" v-model="queryDataNeed.begin_time">
			{{ queryDataNeed.begin_time }}
			<label class="form-label">When does this need end?</label>
			<input type="date" aria-label="Date end" class="form-control mb-2" v-model="queryDataNeed.end_time">
			<label class="form-label">At what percentage?</label>
			<input type="number" aria-label="Percentage" class="form-control mb-2" v-model.number="queryDataNeed.percentage">
			<button v-on:click="createProjectNeed" class="btn btn-gradient" type="button">Save need</button>
		</div>
		<hr />
		<button v-if="'id' in chosenProject" type="submit" class="btn btn-gradient mb-1 mt-4">Save Project</button>
	</form> 
</template>

<script>
export default {
	name: 'FormProject',
	data() {
		return {
			queryDataNeed: {
				id: '06ba4809-f20b-4687-945b-e033a6751fca',
				project_id: this.chosenProject.id,
				count_of_users: Number,
				begin_time: "",
				end_time: "",
				percentage: Number,
				updated_by: this.$store.state.loggeduser.email
			},
			queryDataNeedSkill: {
				id: '06ba4809-f20b-4687-945b-e033a6751fca',
				projectneed_id: '',
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
			chosenNeed: {
				type: Object,
				default() {
					return { 
						id: '',
					}
				}
			},
			availableSkills: {},
			skillLevels: {},
			errors: []
		}
	},
	props: {
		chosenProject: {
			type: Object,
			default() {
				return { id: '' }
			}
		},
	},
	methods: {
		createProject() {
			fetch('/api/projects', {
				method: 'POST',
				headers: {"Content-Type": "application/json"},
				credentials: 'include',
				body: JSON.stringify(this.chosenProject)
			})
			.then((response) => response.json())
			.then((response) => {
				this.chosenProject.id = response.id
				this.queryDataNeed.project_id = response.id
			})
			.catch((errors) => {
				this.errors.push('project-error')
				console.log(errors);
			})
		},
		updateProject() {
			fetch(`/api/projects/${this.chosenProject.id}`, {
				method: 'PUT',
				headers: {"Content-Type": "application/json"},
				credentials: 'include',
				body: JSON.stringify(this.chosenProject)
			})
			.then((response) => response.json())
			.then((response) => {
				this.chosenProject.id = response.id
				this.queryDataNeed.project_id = response.id
			})
			.catch((errors) => {
				console.log(errors);
			})
		},
		createProjectNeed() {
			this.queryDataNeed.begin_time = `${this.queryDataNeed.begin_time}T00:00:00`
			this.queryDataNeed.end_time = `${this.queryDataNeed.end_time}T00:00:00`
			fetch('/api/projectneeds', {
				method: 'POST',
				headers: {"Content-Type": "application/json"},
				credentials: 'include',
				body: JSON.stringify(this.queryDataNeed)
			})
			.then((response) => response.json())
			.then(response => { 
				this.queryDataNeed = response;
				this.queryDataNeedSkill.projectneed_id = response.id
				this.chosenNeed = response
				this.$store.commit('setChosenProject', this.chosenProject.id)
			})
			.catch((errors) => {
				console.log(errors);
			})
		},
		createProjectNeedSkill() {
			fetch('/api/projectskills', {
				method: 'POST',
				headers: {"Content-Type": "application/json"},
				credentials: 'include',
				body: JSON.stringify(this.queryDataNeedSkill)
			})
			.then((response) => response.json())
			.then(response => { 
				this.chosenSkill.id = response.id;
				this.$store.commit('setChosenProject', this.chosenProject.id)
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
		'chosenNeed.id'(newID) {
			this.queryDataNeedSkill.projectneed_id = newID
		},
		'queryDataNeedSkill.skill_id'(newID, oldID) {
			this.getSkillScope(newID)
		},
		'chosenProject.id'(newID, oldID) {
			console.log("Project id changed from " + oldID + " to " + newID)
			if (typeof newID !== 'undefined' && newID.length) {
				this.$store.commit('setChosenProject', newID)
				this.queryDataNeed.project_id = newID 
			} else {
				this.$store.commit('resetChosenProject')
			}
			this.chosenNeed = {}
			this.chosenSkill.id = ''
		}
	},
	computed: {
		filterLevels() {
			if ('skillscope_id' in this.chosenSkill) {
				return this.skillLevels.filter(lvl => lvl.skillscope_id == this.chosenSkill.skillscope_id)
			}
		}
	},
	mounted() {
		console.log("Mounted. chosen project:")
		console.log(this.chosenProject)
		this.getAllSkills()
		this.getAllLevels()
	}
};
</script>