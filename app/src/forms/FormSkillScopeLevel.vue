<template>
	<v-form v-on:submit="saveSkillLevel">
		<div class="mb-2">
			<label class="form-label">Level name</label>
			<error-message name="name" class="error"></error-message>
			<v-field
				v-model="formData.label"
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
				v-model.number="formData.percentage"
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
			scopes: {},
			formData: {
				id: this.chosenLevel.id || undefined,
				label: this.chosenLevel.label || '',
				percentage: this.chosenLevel.percentage || 0,
				skillscope_id: this.chosenScope.id,
				email: '',
			}
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
	methods: {
		isRequired(value) {
			return value ? true : 'This field is required';
		},
		async saveSkillLevel() {
			const level = await this.$api.skills.levels.save(this.formData)
			if (level) this.$emit('formSent')
		},
	},
	async mounted() {
		this.scopes = await this.$api.skills.scopes.get()
	}
};
</script>