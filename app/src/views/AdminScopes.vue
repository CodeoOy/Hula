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
		<div class="d-flex flex-row justify-content-between align-items-start">
			<h2 class="h2">Scopes</h2>
			<button
				class="btn btn-gradient"
				data-bs-toggle="modal"
				data-bs-target="#hulaModalScopes"
				v-on:click="formTitle = 'Add scope', chosenForm = 'Scope', chosenScope = chosenScopeDefault, url='/api/skills/scopes', method='POST'"
			>Add scope</button>
		</div>
		<transition name="fadeHeight">
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
											data-bs-toggle="modal"
											data-bs-target="#hulaModalScopes" 
											v-on:click="formTitle = `Delete ${scope.label}?`, chosenForm = 'Delete', url = `/api/skills/scopes/${scope.id}`, method = 'DELETE'"
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
												data-bs-toggle="modal"
												data-bs-target="#hulaModalScopes" 
												v-on:click="formTitle = `Delete ${lvl.label}?`, chosenForm = 'Delete', url = `/api/skills/levels/${lvl.id}`, method = 'DELETE'"
											><i class="bi-trash-fill me-2"></i></a>
											<a href="#" v-on:click.prevent="chosenLevel = lvl, chosenLevel.swap_direction = 'Better', this.swapLevels(lvl.id);"><i class="bi-caret-up-fill me-1"></i></a>
											<a href="#" v-on:click.prevent="chosenLevel = lvl, chosenLevel.swap_direction = 'Worse', this.swapLevels(lvl.id);"><i class="bi-caret-down-fill me-2"></i></a>
										</div>
									</li>
								</transition-group>
							</td>
						</tr>
					</tbody>
				</table>
			</div>
		</transition>
	</div>
</template>

<script>
	import VModal from '../components/VModal.vue'
	import { Modal } from 'bootstrap'
	import FormSkillScope from '../forms/FormSkillScope.vue'
	import FormSkillScopeLevel from '../forms/FormSkillScopeLevel.vue'
	import FormConfirmAction from '../forms/FormConfirmAction.vue'
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
				skillLevels: [],
			}
		},
		components: {
			VModal,
			FormSkillScope,
			FormSkillScopeLevel,
			FormConfirmAction
		},
		methods: {
			swapLevels(id) {
				fetch(`/api/skills/levels/${id}`, {
					method: 'PUT',
					headers: {"Content-Type": "application/json"},
					credentials: 'include',
					body: JSON.stringify(this.chosenLevel)
				})
				.then(response => { 
					this.getAllLevels()
					return (response.status >= 200 && response.status <= 299) ? response.json() : this.$store.commit('errorHandling', response)
				})
				.catch((errors) => {
					this.$store.commit('errorHandling', errors)
				})
			},
			getAllScopes() {
				fetch('/api/skills/scopes', {method: 'GET'})
				.then(response => { 
					return (response.status >= 200 && response.status <= 299) ? response.json() : this.$store.commit('errorHandling', response)
				})
				.then(response => { 
					this.skillScopes = response;
				})    
				.catch((errors) => {
					this.$store.commit('errorHandling', errors)
				})
			},
			getAllLevels() {
				fetch('/api/skills/levels', {method: 'GET'})
				.then(response => { 
					return (response.status >= 200 && response.status <= 299) ? response.json() : this.$store.commit('errorHandling', response)
				})
				.then(response => { 
					this.skillLevels = response; // Handle 204
				})
				.catch((errors) => {
					this.$store.commit('errorHandling', errors)
				})
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
				this.$store.dispatch('setChosenProject', this.$route.params.id)
				let modal = Modal.getInstance(document.querySelector('#hulaModalSingleProject'))
				modal.hide()
			},
			hideModalUpdate() {
				this.getAllScopes()
				this.getAllLevels()
				let modal = Modal.getInstance(document.querySelector('#hulaModalScopes'))
				modal.hide()
			},
		},
		computed: {
			modalComponent() {
				const components = {
					Scope: FormSkillScope,
					Level: FormSkillScopeLevel,
					Delete: FormConfirmAction
				}
				return components[this.chosenForm]
			}
		},
		mounted() {
			this.getAllLevels()
			this.getAllScopes()
		}
	}
</script>