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
import { computed } from 'vue';
export default {
	name: 'SkillScopeLevel',
	data() {
		return {
			scopes: {}
		};
	},
	components: {
		'VForm': Form,
		'VField': Field,
		ErrorMessage
	},
	props: {
		chosenScope: {},
		chosenLevel: {},
		url: '',
		method: ''
	},	
	setup(props, { emit }) {
		const queryData = computed({
			get: () => {
				if (props.chosenLevel) {
					return {
						email: "",
						skillscope_id: props.chosenScope.id,
						label: props.chosenLevel.label,
						percentage: props.chosenLevel.percentage,
					}
				} else {
					return {
						email: "",
						skillscope_id: props.chosenScope.id,
						label: "",
						percentage: 0,
					}
				}
			},
			/*
			set: (value) => {
				emit("update:chosenScope", {
					...props.chosenScope,
					skillscope_id: value,
				});
			},
			// This remains here for now if it's needed later */
		});
		return {
			queryData,
		};
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
		console.log(this.chosenScope)
	}
};
</script>