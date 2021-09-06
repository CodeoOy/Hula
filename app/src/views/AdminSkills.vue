<template>
	<div>
		<div class="d-sm-flex flex-row justify-content-between align-items-start">
			<h2 class="h2">Skills</h2>
			<div>
				<button class="btn btn-gradient" v-on:click="editCategory()">New category</button>
			</div>
		</div>
		<div class="table-responsive">
			<table class="table table-dark table-striped text-light">
				<thead>
					<tr>
						<th scope="col">Category</th>
						<th scope="col">Skills</th>
					</tr>
				</thead>
				<tbody>
					<tr v-for="category in categories" :key="category.id">
						<td class="hoverable-td">
							<div class="title-actions">
								<span class="title-actions__maintitle">{{ category.label }}</span>
								<div class="title-actions__actions">
									<button class='btn btn-unstyled' v-on:click="editSkill(category)"><i class="bi-plus-circle-fill me-2"></i></button>
									<button class='btn btn-unstyled' v-on:click="editCategory(category)"><i class="bi-pencil-fill me-2"></i></button>
									<button class='btn btn-unstyled' v-on:click="confirmDelete('skill.category', category)"><i class="bi-trash-fill me-2"></i></button>
								</div>
							</div>
						</td>
						<td class="hoverable-td">
							<div class="title-actions" v-for="skill in filterSkills(category.id)" :key="skill" :value="skill.id">
								<span><span class="title-actions__maintitle">{{ skill.label }}</span><span class="title-actions__maintitle--dimmed"> ({{ getSkillScopeLabel(skill.skillscope_id) }})</span></span>
								<div class="title-actions__actions">
									<button class='btn btn-unstyled' v-on:click="editSkill(skill)"><i class="bi-pencil-fill me-2"></i></button>
									<button class='btn btn-unstyled' v-on:click="confirmDelete('skill', skill)"><i class="bi-trash-fill me-2"></i></button>
								</div>
							</div>
						</td>
					</tr>
				</tbody>
			</table>
		</div>
	</div>
</template>

<script>
	import FormSkill from '../forms/FormSkill.vue'
	import FormSkillCategory from '../forms/FormSkillCategory.vue'

	export default {
		name: 'AdminListSkills',
		methods: {
			filterSkills(id) {
				return this.skills.filter(skill => skill.skillcategory_id == id)
			},
			getSkillScopeLabel(id) {
				if (id && this.scopes.length) {
					var scope = this.scopes.find(skillScope => skillScope.id == id)
					return scope.label
				}
			},
			async editCategory(props = {}) {
				const result = await this.$modal({
					title: props.id ? `Edit category: ${props.label}` : 'Add category',
					component: FormSkillCategory,
					props,
				})

				if (result) this.$store.dispatch('getSkillCategories')
			},
			async editSkill(props = {}) {
				const result = await this.$modal({
					title: props.skillcategory_id ? `Edit skill: ${props.label}` : `Add skill to ${props.label}`,
					component: FormSkill,
					props: props.skillcategory_id ? props : { skillcategory_id: props.id }
				})

				if (result) {
					this.$store.dispatch('getSkillCategories')
					this.$store.dispatch('getSkills')
				}
			},
			async confirmDelete(type, data) {
				const success = await this.$confirm.delete(type, data)
				
				if (success) {
					switch (type) {
						case 'skill':
							this.$store.dispatch('getSkills')
							break

						case 'skill.category':
							this.$store.dispatch('getSkillCategories')
							break
					}
				}
			},
		},
		computed: {
			categories() {
				return this.$store.state.skillCategories
			},
			scopes() {
				return this.$store.state.skillScopes
			},
			skills() {
				return this.$store.state.skills
			},
		},
		mounted() {
			this.$store.dispatch('getSkills')
			if (!this.$store.state.skillCategories.length) this.$store.dispatch('getSkillCategories')
			if (!this.$store.state.skillScopes.length) this.$store.dispatch('getSkillScopes')
		}
	}
</script>