<template>
	<v-form v-on:submit="saveSkillCategory">
		<div class="mb-2">
			<label class="form-label">Category label</label>
			<error-message name="name" class="error"></error-message>
			<v-field
				:rules="isRequired"
				as="input"
				type="text"
				name="name"
				class="form-control"
				placeholder="Techs"
				aria-label="Category name"
				v-model="formData.label"
			></v-field>
		</div>
		<div class="mb-2" v-if="categories.length">
			<label class="form-label">Category parent (optional)</label>
			<error-message name="parent" class="error"></error-message>
			<v-field
				as="select"
				name="parent"
				class="form-select"
				aria-label="Category parent"
				v-model="formData.parent_id"
			>
				<option :value="null">No parent</option>
				<option v-for="category in categories" :key="category.label" :value="category.id">
					{{ category.label }}
				</option>
			</v-field>
		</div>
		<button type="submit" class="btn btn-gradient">Submit</button>
	</v-form>   
</template>

<script>
import { Field, Form, ErrorMessage } from 'vee-validate';
export default {
	name: 'SkillCategory',
	data() {
		return {
			categories: {},
			formData: {
				id: this.chosenCategory.id || undefined,
				email: this.chosenCategory.email || '',
				label: this.chosenCategory.label || '',
				parent_id: this.chosenCategory.parent_id || null,
			}
		};
	},
	components: {
		'VForm': Form,
		'VField': Field,
		ErrorMessage
	},
	props: {
		chosenCategory: {},
		url: '',
		method: '',
	},
	methods: {
		isRequired(value) {
			return value ? true : 'This field is required';
		},
		async saveSkillCategory() {
			if (this.formData.parent_id == '') {
				console.log("no parent")
				delete this.formData.parent_id;
			}
			const category = await this.$api.skills.categories.save(this.formData)
			if (category) this.$emit('formSent')
		},
	},
	async mounted() {
		this.categories = await this.$api.getSkillCategories()
	},
};
</script>