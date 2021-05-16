<template>
	<div class="container mt-4">
		<div class="modal fade" v-bind:class="{ 'show db': editing_skills, '': !editing_skills }">
			<div class="modal-dialog">
				<div class="modal-content p-3 rounded-2 content-box bg-dark text-light">
					<div>
						<h2>Create a skill</h2>
						<form v-on:submit="createSkill">
							<div class="input-group">
								<input type="text" aria-label="Label" class="form-control" v-model="skilldata.label" >
								<select class="form-select" id="inputGroupSelect04" aria-label="Example select with button addon" v-model="skilldata.category_id">
									<option v-for="category in categories" :key="category" :value="category.id">
										{{ category.label }}
									</option>
								</select>
							</div>
							<button type="submit" class="btn btn-gradient mb-1">Submit</button>
						</form>
					</div>
				</div>
			</div>
		</div>
		<div class="row gx-4">
			<div class="col-md-4">
				<div class="p-3 mb-4 rounded-2 content-box bg-dark text-light">
					<h1>Welcome {{ this.$store.state.loggeduser.firstname }}!</h1>
					<a href="#" v-on:click="editing_skills = true">Add test skill</a>
				</div>
			</div>
			<div class="col-md-8">
				<div class="p-3 mb-4 rounded-2 content-box bg-dark text-light">
					
				</div>
			</div>
		</div>
	</div>
</template>

<script>
	export default {
		name: 'Admin',
		data() {
			return {
				editing_skills: false,
				skilldata: {
					label: '',
					category_id: '',
					email: this.$store.state.loggeduser.email,
				},
				categories: {}			
			}
		},
		methods: {
			createSkill: function() {
				fetch('http://localhost:8086/api/skill', {
					method: 'POST',
					headers: {"Content-Type": "application/json"},
					credentials: 'include',
					body: JSON.stringify(this.skilldata)
				})
			},
			getSkillCategories: function() {
				fetch('http://localhost:8086/api/skills/categories', {method: 'GET'})
				.then((response) => response.json())
				.then(response => { 
					this.categories = response;
				})    
				.catch((errors) => {
					console.log(errors);
				})
			}
		},
		mounted() {
			this.getSkillCategories()
		}
	}
</script>