<template>
	<form action="#" @submit.prevent="createSkillScopeLevel">
		<p v-if="errorsPresent" class="error">Please fill out label!</p>
		<div class="mb-2">
			<label class="form-label">Level name</label>
			<input class="form-control" type="text" placeholder="Rookie" name="levelname" v-model="querydata.label" />
		</div>
		<div class="mb-2">
			<label class="form-label">Level scope</label>
			<select class="form-select mb-2" id="SkillScope" aria-label="Select scope" v-model="querydata.skillscope_id">
				<option v-for="scope in scopes" :key="scope" :value="scope.id">
					{{ scope.label }}
				</option>
			</select>
		</div>
		<div class="mb-2">
			<label class="form-label">Percentage</label>
			<input type="number" aria-label="Percentage" class="form-control" v-model.number="querydata.percentage">
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
			querydata: {
				email: this.$store.state.loggeduser.email,
				label: "",
				skillscope_id: null,
				percentage: Number,
			},
			scopes: {}
		};
	},
	methods: {
		createSkillScopeLevel: function() {
			fetch('/api/skills/level', {
				method: 'POST',
				headers: {"Content-Type": "application/json"},
				credentials: 'include',
				body: JSON.stringify(this.querydata)
			})
		},
		getSkillScopes: function() {
			fetch('/api/skills/scopes', {method: 'GET'})
			.then((response) => response.json())
			.then(response => { 
				this.scopes = response;
			})    
			.catch((errors) => {
				console.log(errors);
			})
		},
		onSubmit: function() {
			if (this.user.lastname === '') {
				this.errorsPresent = true;
				this.$flashMessage.show({
					type: 'error',
					title: 'Please fill out name.',
					time: 1000
				});
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