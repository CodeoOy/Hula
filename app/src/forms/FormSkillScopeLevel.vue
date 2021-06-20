<template>
	<form v-on:submit="createSkillScopeLevel">
		<p v-if="errorsPresent" class="error">Please fill out label!</p>
		<div class="mb-2">
			<label class="form-label">Level name</label>
			<input class="form-control" type="text" placeholder="Rookie" name="levelname" v-model="queryData.label" />
		</div>
		<div class="mb-2">
			<label class="form-label">Level scope</label>
			<select class="form-select mb-2" id="SkillScope" aria-label="Select scope" v-model="queryData.skillscope_id">
				<option v-for="scope in scopes" :key="scope" :value="scope.id">
					{{ scope.label }}
				</option>
			</select>
		</div>
		<div class="mb-2">
			<label class="form-label">Percentage</label>
			<input type="number" aria-label="Percentage" class="form-control" v-model.number="queryData.percentage">
		</div>
		<button type="submit" class="btn btn-gradient mb-1">Submit</button>
	</form> 
</template>

<script>
export default {
	name: 'SkillScopeLevel',
	data() {
		return {
			errorsPresent: false,
			queryData: {
				email: this.$store.state.loggeduser.email,
				label: "",
				skillscope_id: null,
				percentage: Number,
			},
			scopes: {}
		};
	},
	methods: {
		createSkillScopeLevel() {
			fetch('/api/skills/levels', {
				method: 'POST',
				headers: {"Content-Type": "application/json"},
				credentials: 'include',
				body: JSON.stringify(this.queryData)
			})
			.catch((errors) => {
				console.log(errors);
			})
		},
		getSkillScopes() {
			fetch('/api/skills/scopes', {method: 'GET'})
			.then((response) => response.json())
			.then(response => { 
				this.scopes = response;
			})    
			.catch((errors) => {
				console.log(errors);
			})
		},
		onSubmit() {
			if (this.user.lastname === '') {
				this.errorsPresent = true;
			} else {
				this.$emit('formsent', this.user);
			}
		},
	},
	mounted() {
		this.getSkillScopes()
	}
};
</script>