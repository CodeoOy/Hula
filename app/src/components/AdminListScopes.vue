<template>
	<div>
		<VModal :modalTitle="formTitle" :modalID="'Scopes'" ref='hulaModalScopes'>
			<component 
				:is='modalComponent' 
				:chosenScope="chosenScope" 
				:url="url"
				:method="method"
				v-on:form-sent="hideModalUpdate"
			/>
		</VModal>
		<div class="d-flex flex-row justify-content-between align-items-start">
			<h2>Scopes</h2>
			<button
				class="btn btn-gradient"
				data-bs-toggle="modal"
				data-bs-target="#hulaModalScopes"
				v-on:click="formTitle = 'Add scope', chosenForm = 'Scope', chosenScope = chosenScopeDefault, url='/api/skills/scopes', method='POST'"
			>Add scope</button>
		</div>
		<transition name="fadeHeight">
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
										v-on:click="formTitle = 'Add Level', chosenForm = 'Level', chosenScope = scope, url='/api/skills/levels', method='POST'"
									><i class="bi-plus-circle-fill me-2"></i></a>
									<a 
										href="#"
										data-bs-toggle="modal"
										data-bs-target="#hulaModalScopes"
										v-on:click="formTitle = 'Edit Scope', chosenForm = 'Scope', chosenScope = scope, url=`/api/skills/scopes/${scope.id}`, method='PUT'"
									><i class="bi-pencil-fill me-2"></i></a>
									<a href="#" v-on:click.prevent="this.deleteScope(scope.id)"><i class="bi-trash-fill me-2"></i></a>
								</div>
							</div>
						</td>
						<td class="hoverable-td">
							<div class="title-actions" v-for="lvl in filterLevels(scope.id)" :key="lvl" :value="lvl.id">
								<span class="title-actions__maintitle">{{ lvl.index }}: {{ lvl.label }} - {{ lvl.percentage }}</span>
								<div class="title-actions__actions">
									<a 
										href="#"
										:data-scope-id="lvl.id" 
										:data-scope-name="lvl.label" 
										data-bs-toggle="modal"
										data-bs-target="#hulaModalScopes"
										title="Edit level" 
										v-on:click="chosenScope=scope, formTitle=lvl.label, chosenForm='Level', url=`/api/skills/levels/${lvl.id}`, method='PUT'"
									><i class="bi-pencil-fill me-2"></i></a>
									<a href="#" v-on:click.prevent="this.deleteLevel(lvl.id)"><i class="bi-trash-fill me-2"></i></a>
								</div>
							</div>
						</td>
					</tr>
				</tbody>
			</table>
		</transition>
	</div>
</template>

<script>
	import VModal from '../components/VModal.vue'
	import FormSkillScope from '../forms/FormSkillScope.vue'
	import FormSkillScopeLevel from '../forms/FormSkillScopeLevel.vue'
	import FormGeneralRename from '../forms/FormGeneralRename.vue'
	export default {
		name: 'AdminListSkills',
		data () {
			return {
				formTitle: '',
				chosenForm: '',
				url: '',
				method: '',
				chosenScope: {},
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
			FormGeneralRename
		},
		methods: {
			getAllScopes() {
				fetch('/api/skills/scopes', {method: 'GET'})
				.then((response) => response.json())
				.then(response => { 
					this.skillScopes = response;
				})    
				.catch((errors) => {
					console.log(errors);
				})
			},
			getAllLevels() {
				fetch('/api/skills/levels', {method: 'GET'})
				.then((response) => response.json())
				.then(response => { 
					this.skillLevels = response;
				})
				.catch((errors) => {
					console.log(errors);
				})
			},
			deleteScope(id) {
				fetch(`/api/skills/scopes/${id}`, {method: 'DELETE'})
				.then(response => { 
					if (response.ok) {
						this.$flashMessage.show({
							type: 'success',
							title: 'Scope removed',
							time: 1000
						});
						this.getAllScopes()
						this.getAllLevels()
					}
				})    
				.catch((errors) => {
					console.log(errors);
				})
			},
			deleteLevel(id) {
				fetch(`/api/skills/levels/${id}`, {method: 'DELETE'})
				.then(response => { 
					if (response.ok) {
						this.$flashMessage.show({
							type: 'success',
							title: 'Level removed',
							time: 1000
						});
						this.getAllScopes()
						this.getAllLevels()
					}
				})    
				.catch((errors) => {
					console.log(errors);
				})
			},
			filterLevels(id) {
				return this.skillLevels.filter(lvl => lvl.skillscope_id == id)
			},
			hideModalUpdate() {
				this.getAllScopes()
				this.getAllLevels()
				let modalToHide = document.getElementById('hulaModalScopes') 
				console.log(modalToHide)
				console.log(this.$refs.hulaModalScopes)
				this.$refs['hulaModalScopes'].hide()
				//$('#hulaModalScopes').modal('hide');
			}
		},
		computed: {
			modalComponent() {
				const components = {
					Scope: FormSkillScope,
					Level: FormSkillScopeLevel
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