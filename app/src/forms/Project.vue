<template>
	<div>
		<form>
			<div class="mb-2">
				<p v-if="errorsPresent" class="error">Please fill out label!</p>
				<div class="mb-2">
					{{ querydata_project.id }}
					<label class="form-label">Project name</label>
					<input class="form-control" type="text" placeholder="Languages" name="label" v-model="querydata_project.name" />
				</div>
				<button v-on:click="createProject" class="btn btn-gradient" type="button">Save</button>
			</div>
			<div class="input-group">
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
				<button class="btn btn-gradient" type="button">Add</button>
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
			querydata_skill: {
				project_id: '',
				skill_id: '',
				skillscopelevel_id: '',
				min_years: 1,
				max_years: 10,
				countofusers: Number,
				begin_time: Date,
				end_time: Date,
				percentage: Number,
			},
			chosenskills: {}	
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
				this.$flashMessage.show({
					type: 'success',
					title: 'Successfully logged in',
					time: 1000
				});
			})
		},
		createProjectSkill: function() {
			fetch('/api/projectskills', {
				method: 'PUT',
				headers: {"Content-Type": "application/json"},
				credentials: 'include',
				body: JSON.stringify(this.querydata)
			})
		}
	},
};
</script>