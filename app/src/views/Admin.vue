<template>
	<div class="container mt-4">
		<div class="modal fade" v-bind:class="{ 'show db': editing_skills, '': !editing_skills }">
			<div class="modal-dialog">
				<div class="modal-content p-3 rounded-2 content-box bg-dark text-light">
					<div>
						<h2>Add a skill</h2>
						<div class="input-group">
							<select class="form-select" id="inputGroupSelect04" aria-label="Example select with button addon">
								<option selected>Choose...</option>
								<option value="1">One</option>
								<option value="2">Two</option>
								<option value="3">Three</option>
							</select>
							<input type="number" aria-label="Years" class="form-control">
							<button class="btn btn-outline-secondary" type="button">Button</button>
						</div>
						<a href="#" v-on:click="editing_skills = false" class="btn btn-gradient">Done</a>
					</div>
				</div>
			</div>
		</div>
		<div class="row gx-4">
			<div class="col-md-4">
				<div class="p-3 mb-4 rounded-2 content-box bg-dark text-light">
					<h1>Welcome {{ user.firstname }}!</h1>
					<a href="#" v-on:click="addExistingSkill, editing_skills = true">Add test skill</a>
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
				user: {},
				editing_skills: false,
				
			}
		}
	}
</script>