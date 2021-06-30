<template>
	<v-form v-on:submit="createUpdateSkillScopeLevel">
		<div class="mb-2">
			<label class="form-label">Level name</label>
			<error-message name="name" class="error"></error-message>
			<v-field
				v-model="queryData.label"
				:rules="isRequired"
				as="input"
				type="text"
				name="name"
				class="form-control"
				placeholder="Rookie"
				aria-label="Level name"
			></v-field>
		</div>
		<div class="mb-2" v-if="method == 'POST'">
			<label class="form-label">Level scope</label>
			<error-message name="scope" class="error"></error-message>
			<v-field
				v-model="queryData.skillscope_id"
				:rules="isRequired"
				as="select"
				name="scope"
				class="form-select"
				aria-label="Level scope"
			>
				<option v-for="scope in scopes" :key="scope" :value="scope.id">
					{{ scope.label }}
				</option>
			</v-field>
		</div>
		<div class="mb-2">
			<label class="form-label">Percentage</label>
			<error-message name="percentage" class="error"></error-message>
			<v-field
				v-model.number="queryData.percentage"
				:rules="isRequired"
				as="input"
				type="number"
				name="percentage"
				class="form-control"
				aria-label="Level percentage"
			></v-field>
		</div>
		<button type="submit" class="btn btn-gradient mb-1">Submit</button>
	</v-form> 
</template>

<script>
import { Field, Form, ErrorMessage } from 'vee-validate';
export default {
	name: 'SkillScopeLevel',
	data() {
		return {
			queryData: {
				email: this.$store.state.loggeduser.email,
				label: "",
				skillscope_id: '00000000-0000-0000-0000-000000000000',
				percentage: 0,
			},
			scopes: {}
		};
	},
	components: {
		'VForm': Form,
		'VField': Field,
		ErrorMessage
	},
	props: {
		chosenSkill: {},
		url: '',
		method: ''
	},	
	methods: {
		isRequired(value) {
			return value ? true : 'This field is required';
		},
		createUpdateSkillScopeLevel() {
			fetch(this.url, {
				method: this.method,
				headers: {"Content-Type": "application/json"},
				credentials: 'include',
				body: JSON.stringify(this.queryData)
			})
			.then(() => {
				this.$emit('formSent')
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
		}
	},
	mounted() {
		this.getSkillScopes()
	}
};
</script>