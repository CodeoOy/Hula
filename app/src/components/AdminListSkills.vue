<template>
	<div>
		<VModal :modalTitle="formTitle" :modalID="'Skills'">
			<component 
				:is='modalComponent' 
				:chosenSkill="chosenSkill" 
				:url="url"
				:method="method"
				v-on:form-sent="hideModalUpdate"
			/>
		</VModal>
		<div class="d-flex flex-row justify-content-between align-items-start">
			<h2>Skills</h2>
			<button
				class="btn btn-gradient"
				data-bs-toggle="modal"
				data-bs-target="#hulaModalSkills"
				v-on:click="formTitle = 'Add skill', chosenForm = 'CreateSkill', chosenSkill = chosenSkillDefault, url='/api/skills', method='POST'"
			>Add skill</button>
		</div>
		<transition name="fadeHeight">
			<table class="table table-dark table-striped text-light">
				<thead>
					<tr>
						<th scope="col">Skill name</th>
						<th scope="col">Category</th>
					</tr>
				</thead>
				<tbody>
					<tr v-for="skill in skills" :key="skill.id">
						<td class="hoverable-td">
							<div class="title-actions">
								<span class="title-actions__maintitle">{{ skill.label }}</span>
								<div class="title-actions__actions">
									<a 
										href="#"
										data-bs-toggle="modal"
										data-bs-target="#hulaModalSkills"
										v-on:click="formTitle=skill.label, chosenForm = 'CreateSkill', chosenSkill = skill, url=`/api/skills/${skill.id}`, method='PUT'"
									><i class="bi-pencil-fill me-2"></i></a>
									<a href="#" v-on:click.prevent="this.deleteSkill(skill.id)"><i class="bi-trash-fill me-2"></i></a>
								</div>
							</div>
						</td>
						<td>{{ skill.skillcategory_id }}</td>
					</tr>
				</tbody>
			</table>
		</transition>
	</div>
</template>

<script>
	import VModal from '../components/VModal.vue'
	import { Modal } from 'bootstrap'
	import FormAddSkill from '../forms/FormAddSkill.vue'
	import FormSkill from '../forms/FormSkill.vue'
	import FormSkillCategory from '../forms/FormSkillCategory.vue'
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
				chosenSkill: {},
				chosenSkillDefault: {
					label: '',
					skillcategory_id: null,
					skillscope_id: null,
				},
				skills: [],
			}
		},
		components: {
			VModal,
			FormSkill,
			FormSkillCategory,
			FormSkillScope,
			FormSkillScopeLevel,
			FormAddSkill,
			FormGeneralRename
		},
		methods: {
			getAllSkills() {
				fetch('/api/skills', {method: 'GET'})
				.then((response) => response.json())
				.then(response => { 
					this.skills = response;
				})    
				.catch((errors) => {
					console.log(errors);
				})
			},
			deleteSkill(id) {
				fetch(`/api/skills/${id}`, {method: 'DELETE'})
				.then(response => { 
					if (response.ok) {
						this.$flashMessage.show({
							type: 'success',
							title: 'Skill removed',
							time: 1000
						});
						this.getAllSkills()
					}
				})    
				.catch((errors) => {
					console.log(errors);
				})
			},
			hideModalUpdate() {
				this.getAllSkills()
				let modal = Modal.getInstance(document.querySelector('#hulaModalSkills'))
				modal.hide()
			}
		},
		computed: {
			modalComponent() {
				const components = {
					CreateSkill: FormSkill,
					Category: FormSkillCategory,
					Scope: FormSkillScope,
					Level: FormSkillScopeLevel
				}
				return components[this.chosenForm]
			}
		},
		mounted() {
			this.getAllSkills()
		}
	}
</script>