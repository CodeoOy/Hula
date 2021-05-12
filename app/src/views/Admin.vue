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
								<!--<select class="form-select" id="inputGroupSelect04" aria-label="Example select with button addon">
									<option selected>Choose...</option>
									<option value="1">Years</option>
									<option value="2">Levels</option>
									<option value="3">Something else</option>
								</select>-->
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
					email: this.$store.state.loggeduser.email,
				}				
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
		},
		mounted() {

		}
	}
</script>