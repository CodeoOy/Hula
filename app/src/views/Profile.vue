<template>
	<div class="container mt-4">
		<div class="modal fade" v-bind:class="{ 'show db': editing_skills, '': !editing_skills }">
			<div class="modal-dialog">
				<div class="modal-content p-3 rounded-2 content-box bg-dark text-light">
					<div>
						<h2>Add a skill</h2>
						{{ skilldata.skill_id }}
						<form action="#" @submit.prevent="addExistingSkill">
							<select class="form-select" id="AddExistingSkill" aria-label="Example select with button addon" v-model="skilldata.skill_id">
								<option v-for="avskill in available_skills" :key="avskill" :value="avskill.id">
									{{ avskill.label }}
								</option>
							</select>
							<select v-if="chosenskill.skillscope_id" class="form-select" id="AddExistingSkill" aria-label="Example select with button addon" v-model="skilldata.skillscopelevel_id">
								<option v-for="levelres in filterLevels" :key="levelres" :value="levelres.id">
									{{ levelres.label }}
								</option>
							</select>
							<input type="number" aria-label="Years" class="form-control" v-model.number="skilldata.years">
							<button class="btn btn-outline-secondary" type="button">Button</button>
						<button type="submit" class="btn btn-gradient mb-1">Submit</button>
						</form>
					</div>
				</div>
			</div>
		</div>
		<div class="row gx-4">
			<div class="col-md-4">
				<div class="p-3 mb-4 rounded-2 content-box bg-dark text-light">
					<h1>{{ user.firstname }} {{ user.lastname }}</h1>
					<p>{{ user.email }}</p>
					<p>{{ user.telephone }}</p>
					<p>{{ user.id }}</p>
					<img src="" alt="">
				</div>
			</div>
			<div class="col-md-8">
				<div class="p-3 mb-4 rounded-2 content-box bg-dark text-light">
					<h2>Professional profile</h2>
					<h3>Basic info</h3>
					<a href="#" v-on:click="editing_info = true">Edit your info</a>
					<p>Available: {{ user.available }}</p>
					<transition name="fadeHeight">
						<UserForm :user='user' v-if="editing_info == true" v-on:formsent="editing_info = false, updateUser()"/>
					</transition>
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
							<tr v-for="skill in user.skills" :key="skill.id">
								<td>{{ skill.skill_label }}</td>
								<td>{{ skill.years }}</td>
								<td>Edit - Delete</td>
							</tr>
						</tbody>
					</table>
					<a href="#" v-on:click="editing_skills = true">Add test skill</a>
				</div>
			</div>
		</div>
	</div>
</template>

<script>
	import UserForm from '../forms/UserBasicInfo.vue'
	export default {
		name: 'Profile',
		data() {
			return {
				user: {},
				editing_info: false,
				editing_skills: false,
				chosenskill: {
					id: '',
					skillscope_id: '',
				},
				skilldata: {
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
		components: {
			'UserForm': UserForm
		},
		methods: {
			addExistingSkill: function() {
				console.log(this.skilldata)
				fetch(`http://localhost:8086/api/userskill/${this.user.id}`, {
					method: 'POST',
					headers: {"Content-Type": "application/json"},
					credentials: 'include',
					body: JSON.stringify(this.skilldata)
				})
				.then(() => {
					this.$store.commit('setUser', this.user.id)
				})
			},
			updateUser: function() { // This is not working. Make the user update happen in some other way.
				fetch(`http://localhost:8086/api/user/${this.user.id}`, {
					method: 'PUT',
					headers: {"Content-Type": "application/json"},
					credentials: 'include',
					body: JSON.stringify(this.user)
				})
				.then(() => {
					this.$store.commit('setUser', this.user.id)
				})
			},
			getAllSkills: function() {
				fetch('http://localhost:8086/api/skills', {method: 'GET'})
				.then((response) => response.json())
				.then(response => { 
					this.available_skills = response;
				})    
				.catch((errors) => {
					console.log(errors); // This gives unexpected end of json
				})
			},
			getAllLevels: function() {
				fetch('http://localhost:8086/api/skills/levels', {method: 'GET'})
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
				console.log(this.skilldata.id)
			}
		},
		watch: {
			'skilldata.skill_id': function(newID, oldID) {
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
			//this.$store.commit('updateUser')
			this.user = this.$store.state.loggeduser
			this.getAllSkills()
			this.getAllLevels()
		}
	}
</script>