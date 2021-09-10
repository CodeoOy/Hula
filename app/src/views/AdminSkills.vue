<template>
	<div class='card shadow' :class='$colorScheme.card'>
		<div class='card-header'>
			<div class="d-sm-flex flex-row justify-content-between align-items-center">
				<h1 class="h3 mb-0">Skills &amp; Categories</h1>
				<button class="btn btn-primary gradient" v-on:click="editCategory()">New category</button>
			</div>
		</div>
		<div class='card-body'>
			<div class="table-responsive">
				<table class="table table-striped" :class='$colorScheme.table'>
					<thead>
						<tr>
							<th scope="col">Category</th>
							<th scope="col">Skills</th>
						</tr>
					</thead>
					<tbody>
						<tr v-for="category in categories" :key="category.id">
							<td>
								<div class='d-flex justify-content-between m-3'>
									<div>{{ category.label }}</div>
									<div class="td-actions">
										<button class='btn btn-unstyled' v-on:click="editSkill(category)"><i class="bi-plus-circle-fill me-2"></i></button>
										<button class='btn btn-unstyled' v-on:click="editCategory(category)"><i class="bi-pencil-fill me-2"></i></button>
										<button class='btn btn-unstyled' v-on:click="confirmDelete('skill.category', category)"><i class="bi-trash-fill me-2"></i></button>
									</div>
								</div>
							</td>
							<td>
								<ul class='list-group list-group-flush list-group-transparent ms-n3 my-2'>
									<li v-for="skill in filterSkills(category.id)" :key="skill" class='list-group-item d-flex justify-content-between'>
										<div>{{ skill.label }} <small class="fw-light text-muted">({{ getSkillScopeLabel(skill.skillscope_id) }})</small></div>
										<div class="td-actions">
											<button class='btn btn-unstyled' v-on:click="editSkill(skill)"><i class="bi-pencil-fill me-2"></i></button>
											<button class='btn btn-unstyled' v-on:click="confirmDelete('skill', skill)"><i class="bi-trash-fill me-2"></i></button>
										</div>
									</li>
								</ul>
							</td>
						</tr>
					</tbody>
				</table>
			</div>
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