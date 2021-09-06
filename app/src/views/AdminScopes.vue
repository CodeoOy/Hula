<template>
	<div>
		<div class="d-sm-flex flex-row justify-content-between align-items-start">
			<h2 class="h2">Scopes</h2>
			<button class="btn btn-gradient" v-on:click="editScope()">Add scope</button>
		</div>
		<div class="table-responsive">
			<table class="table table-dark table-striped text-light">
				<thead>
					<tr>
						<th scope="col">Scope name</th>
						<th scope="col">Levels</th>
					</tr>
				</thead>
				<tbody>
					<tr v-for="scope in skillScopes" :key="scope.id">
						<td class="hoverable-td">
							<div class="title-actions">
								<span class="title-actions__maintitle">{{ scope.label }}</span>
								<div class="title-actions__actions">
									<button class='btn btn-unstyled' v-on:click="editLevel(scope)"><i class="bi-plus-circle-fill me-2"></i></button>
									<button class='btn btn-unstyled' v-on:click="editScope(scope)"><i class="bi-pencil-fill me-2"></i></button>
									<button class='btn btn-unstyled' v-on:click="confirmDelete('skill.scope', scope)"><i class="bi-trash-fill me-2"></i></button>
								</div>
							</div>
						</td>
						<td class="hoverable-td">
							<transition-group name="flip-list" tag="ul">
								<li class="title-actions" v-for="lvl in filterLevels(scope.id)" :key="lvl.id" :value="lvl.id">
									<span class="title-actions__maintitle">{{ lvl.index }}: {{ lvl.label }} - {{ lvl.percentage }}</span>
									<div class="title-actions__actions">
										<button class='btn btn-unstyled' v-on:click="editLevel(lvl)"><i class="bi-pencil-fill me-2"></i></button>
										<button class='btn btn-unstyled' v-on:click="confirmDelete('skill.level', lvl)"><i class="bi-trash-fill me-2"></i></button>
										<button class='btn btn-unstyled' v-on:click="swapLevels({ ...lvl, swap_direction: 'Better' })"><i class="bi-caret-up-fill me-1"></i></button>
										<button class='btn btn-unstyled' v-on:click="swapLevels({ ...lvl, swap_direction: 'Worse' })"><i class="bi-caret-down-fill me-2"></i></button>
									</div>
								</li>
							</transition-group>
						</td>
					</tr>
				</tbody>
			</table>
		</div>
	</div>
</template>

<script>
	import FormSkillScope from '../forms/FormSkillScope.vue'
	import FormSkillScopeLevel from '../forms/FormSkillScopeLevel.vue'

	export default {
		name: 'AdminListSkills',
		methods: {
			swapLevels(data) {
				this.$store.dispatch('saveSkillLevel', data)
			},
			filterLevels(id) {
				let levels = this.skillLevels.filter(lvl => lvl.skillscope_id == id)
				return levels.sort(function(a, b) {
					let keyA = a.index
					let keyB = b.index
					if (keyA < keyB) return 1;
  					if (keyA > keyB) return -1;
  					return 0;
				})
			},
			async editScope(props = {}) {
				const result = await this.$modal({
					title: props.id ? `Edit scope: ${props.label}` : 'Add scope',
					component: FormSkillScope,
					props,
				})

				if (result) this.$store.dispatch('getSkillScopes')
			},
			async editLevel(props = {}) {
				const result = await this.$modal({
					title: props.skillscope_id ? `Edit level: ${props.label}` : `Add level to ${props.label}`,
					component: FormSkillScopeLevel,
					props: props.skillscope_id ? props : { skillscope_id: props.id },
				})

				if (result) {
					this.$store.dispatch('getSkillLevels')
				}
			},
			async confirmDelete(type, data) {
				const success = await this.$confirm.delete(type, data)
				if (success) {
					switch (type) {
						case 'skill.scope':
							this.$store.dispatch('getSkillScopes')
							break
						
						case 'skill.level':
							this.$store.dispatch('getSkillLevels')
							break
					}
				}
			},
		},
		computed: {
			skillScopes() {
				return this.$store.state.skillScopes
			},
			skillLevels() {
				return this.$store.state.skillLevels
			},
		},
		mounted() {
			if (!this.$store.state.skillLevels.length) this.$store.dispatch('getSkillLevels')
			if (!this.$store.state.skillScopes.length) this.$store.dispatch('getSkillScopes')
		}
	}
</script>
