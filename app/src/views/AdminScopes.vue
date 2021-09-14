<template>
	<div class='card shadow' :class='$colorScheme.card'>
		<div class='card-header'>
			<div class="d-flex justify-content-between align-items-center flex-wrap">
				<h1 class="h3 flex-grow-1 mb-0">Scopes &amp; Levels</h1>
				<button class="btn btn-primary gradient" v-on:click="editScope()">Add scope</button>
			</div>
		</div>
		<div class='card-body'>
			<div v-if='skillScopes.length'>
				<table class="table table-striped table-stack-mobile" :class='$colorScheme.table'>
					<thead>
						<tr>
							<th scope="col" class='ps-3'>Scope name</th>
							<th scope="col">Levels</th>
						</tr>
					</thead>
					<tbody>
						<tr v-for="scope in skillScopes" :key="scope.id">
							<td>
								<div class='table-stack-mobile-cell'>
									<div class="d-flex justify-content-between m-md-2">
										<div>{{ scope.label }}</div>
										<div class='td-actions'>
											<button class='btn btn-unstyled' v-on:click="editLevel(scope)"><i class="bi-plus-circle-fill me-2"></i></button>
											<button class='btn btn-unstyled' v-on:click="editScope(scope)"><i class="bi-pencil-fill me-2"></i></button>
											<button class='btn btn-unstyled' v-on:click="confirmDelete('skill.scope', scope)"><i class="bi-trash-fill"></i></button>
										</div>
									</div>
								</div>
							</td>
							<td>
								<div class='table-stack-mobile-cell p-0 pe-md-2'>
									<transition-group name="flip-list" tag="ul" class='list-group list-group-flush list-group-transparent'>
										<li v-for="lvl in filterLevels(scope.id)" :key="lvl.id" class="list-group-item d-flex justify-content-between ps-4 pe-2 px-md-0">
											<div>{{ lvl.label }} <small class="fw-light text-muted">({{ lvl.percentage }}%)</small></div>
											<div class='td-actions'>
												<button class='btn btn-unstyled' v-on:click="swapLevels({ ...lvl, swap_direction: 'Better' })"><i class="bi-caret-up-fill me-1"></i></button>
												<button class='btn btn-unstyled' v-on:click="swapLevels({ ...lvl, swap_direction: 'Worse' })"><i class="bi-caret-down-fill me-2"></i></button>
												<button class='btn btn-unstyled' v-on:click="editLevel(lvl)"><i class="bi-pencil-fill me-2"></i></button>
												<button class='btn btn-unstyled' v-on:click="confirmDelete('skill.level', lvl)"><i class="bi-trash-fill"></i></button>
											</div>
										</li>
									</transition-group>
								</div>
							</td>
						</tr>
					</tbody>
				</table>
			</div>
			<div v-else>
				<div class='fs-3 fw-light text-muted text-center p-4'>No skill scopes</div>
			</div>
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
		activated() {
			this.$store.dispatch('getSkillLevels')
			this.$store.dispatch('getSkillScopes')
		}
	}
</script>
