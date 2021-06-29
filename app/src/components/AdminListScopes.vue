<template>
	<div>
		<VModal :modalTitle="formTitle" :modalID="'Scopes'">
			<component 
				:is='modalComponent' 
				:chosenSkill="chosenSkill" 
				:url="url"
				:method="method"
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
						<th scope="col">Actions</th>
					</tr>
				</thead>
				<tbody>
					<tr v-for="scope in scopes" :key="scope.id">
						<td>{{ scope.label }}</td>
						<td>Levels here</td>
						<td>
							<a
								href="#" 
								:data-scope-id="scope.id" 
								:data-scope-name="scope.label" 
								data-bs-toggle="modal" 
								data-bs-target="#hulaModalScopes" 
								v-on:click="chosenScope=scope, formTitle=scope.label, chosenForm='CreateScope', url=`/api/skills/scopes/${scope.id}`, method='PUT'"
								class="me-2"
							>Edit</a>
							<a href="#" v-on:click.prevent="this.deletescope(scope.id)">Delete</a>
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
				scopes: [],
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
					this.scopes = response;
				})    
				.catch((errors) => {
					console.log(errors);
				})
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
			this.getAllScopes()
		}
	}
</script>