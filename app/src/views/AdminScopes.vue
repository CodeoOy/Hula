<template>
	<div>
		<VModal :modalTitle="formTitle" :modalID="'Scopes'" v-on:modal-hidden="chosenForm = ''">
			<component 
				:is='modalComponent' 
				:chosenScope="chosenScope"
				:chosenLevel="chosenLevel"
				:url="url"
				:method="method"
				v-on:form-sent="hideModalUpdate"
			/>
		</VModal>
		<div class="d-sm-flex flex-row justify-content-between align-items-start">
			<h2 class="h2">Scopes</h2>
			<button
				class="btn btn-gradient"
				data-bs-toggle="modal"
				data-bs-target="#hulaModalScopes"
				v-on:click="formTitle = 'Add scope', chosenForm = 'Scope', chosenScope = chosenScopeDefault, url='/api/skills/scopes', method='POST'"
			>Add scope</button>
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
									<a 
										href="#"
										data-bs-toggle="modal"
										data-bs-target="#hulaModalScopes"
										v-on:click="formTitle = `Add level to ${scope.label}`, chosenForm = 'Level', chosenScope = scope, url='/api/skills/levels', method='POST'"
									><i class="bi-plus-circle-fill me-2"></i></a>
									<a 
										href="#"
										data-bs-toggle="modal"
										data-bs-target="#hulaModalScopes"
										v-on:click="formTitle = `Edit ${scope.label}`, chosenForm = 'Scope', chosenScope = scope, url=`/api/skills/scopes/${scope.id}`, method='PUT'"
									><i class="bi-pencil-fill me-2"></i></a>
									<a
										href="#"
										v-on:click.prevent="confirmDelete('skill.scope', scope)"
									><i class="bi-trash-fill me-2"></i></a>
								</div>
							</div>
						</td>
						<td class="hoverable-td">
							<transition-group name="flip-list" tag="ul">
								<li class="title-actions" v-for="lvl in filterLevels(scope.id)" :key="lvl.id" :value="lvl.id">
									<span class="title-actions__maintitle">{{ lvl.index }}: {{ lvl.label }} - {{ lvl.percentage }}</span>
									<div class="title-actions__actions">
										<a 
											href="#"
											:data-scope-id="lvl.id" 
											:data-scope-name="lvl.label" 
											data-bs-toggle="modal"
											data-bs-target="#hulaModalScopes"
											title="Edit level" 
											v-on:click="chosenScope = scope, formTitle = lvl.label, chosenForm = 'Level', chosenLevel = lvl, url = `/api/skills/levels/${lvl.id}`, method = 'PUT'"
										><i class="bi-pencil-fill me-2"></i></a>
										<a
											href="#"
											v-on:click.prevent="confirmDelete('skill.level', lvl)"
										><i class="bi-trash-fill me-2"></i></a>
										<a href="#" v-on:click.prevent="swapLevels({ ...lvl, swap_direction: 'Better' })"><i class="bi-caret-up-fill me-1"></i></a>
										<a href="#" v-on:click.prevent="swapLevels({ ...lvl, swap_direction: 'Worse' })"><i class="bi-caret-down-fill me-2"></i></a>
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
	import VModal from '../components/VModal.vue'
	import { Modal } from 'bootstrap'
	import FormSkillScope from '../forms/FormSkillScope.vue'
	import FormSkillScopeLevel from '../forms/FormSkillScopeLevel.vue'
	export default {
		name: 'AdminListSkills',
		data () {
			return {
				formTitle: '',
				chosenForm: '',
				url: '',
				method: '',
				chosenScope: {},
				chosenLevel: {},
				chosenScopeDefault: {
					label: '',
				},
				skillScopes: [],
			}
		},
		components: {
			VModal,
			FormSkillScope,
			FormSkillScopeLevel,
		},
		methods: {
			swapLevels(data) {
				this.$store.dispatch('saveSkillLevel', data)
			},
			async getSkillScopes() {
				this.skillScopes = await this.$api.skills.scopes.get()
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
			hideModalUpdate() {
				this.getSkillScopes()
				this.$store.dispatch('getSkillLevels')
				let modal = Modal.getInstance(document.querySelector('#hulaModalScopes'))
				modal.hide()
			},
			async confirmDelete(type, data) {
				const success = await this.$confirm.delete(type, data)
				if (success) {
					switch (type) {
						case 'skill.scope':
							this.getSkillScopes()
							break
						
						case 'skill.level':
							this.$store.dispatch('getSkillLevels')
							break
					}
				}
			},
		},
		computed: {
			skillLevels() {
				return this.$store.state.skillLevels
			},
			modalComponent() {
				const components = {
					Scope: FormSkillScope,
					Level: FormSkillScopeLevel,
				}
				return components[this.chosenForm]
			}
		},
		mounted() {
			this.$store.dispatch('getSkillLevels')
			this.getSkillScopes()
		}
	}
</script>
