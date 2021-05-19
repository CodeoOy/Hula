<template>
	<form action="#" @submit.prevent="createSkillLevel">
		<div class="mb-2">
			<label class="form-label">Level name</label>
			<input class="form-control" type="text" placeholder="Languages" name="label" v-model="querydata.label" />
		</div>
		<select class="form-select mb-2" id="SkillLevel" aria-label="Example select with button addon" v-model="querydata.skillscope_id">
			<option v-for="scope in scopes" :key="scope" :value="scope.id">
				{{ scope.label }}
			</option>
		</select>
		<input type="number" aria-label="Percentage" class="form-control" v-model.number="querydata.percentage">
		<button type="submit" class="btn btn-gradient mb-1">Submit</button>
	</form> 
</template>

<script>
export default {
	name: 'SkillScopeLevel',
	props: {	
		user: {}
	},
	data() {
		return {
			errorsPresent: false,
			querydata: {
				email: this.$store.state.loggeduser.email,
				label: "",
				skillscope_id: null,
				index: 2,
				percentage: Number,
			},
			scopes: {}
		};
	},
	methods: {
		createSkillLevel: function() {
			fetch('http://localhost:8086/api/skills/level', {
				method: 'POST',
				headers: {"Content-Type": "application/json"},
				credentials: 'include',
				body: JSON.stringify(this.querydata)
			})
		},
		getSkillScopes: function() {
			fetch('http://localhost:8086/api/skills/scopes', {method: 'GET'})
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