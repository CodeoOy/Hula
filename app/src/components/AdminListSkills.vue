<template>
	<div>
		<VModal :modalTitle="formTitle" :modalID="'Skills'">
			<component 
				:is='modalComponent' 
				:chosenSkill="chosenSkill" 
				:url="url"
				:method="method"
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
						<th scope="col">#</th>
						<th scope="col">Skill name</th>
						<th scope="col">Category</th>
						<th scope="col">Actions</th>
					</tr>
				</thead>
				<tbody>
					<tr v-for="(skill, index) in skills" :key="skill.id">
						<th scope="row">{{ index + 1 }}</th>
						<td>{{ skill.label }}</td>
						<td>{{ skill.skillcategory_id }}</td>
						<td>
							<a
								href="#" 
								:data-skill-id="skill.id" 
								:data-skill-name="skill.label" 
								data-bs-toggle="modal" 
								data-bs-target="#hulaModalSkills" 
								v-on:click="chosenSkill=skill, formTitle=skill.label, chosenForm='CreateSkill', url=`/api/skills/${skill.id}`, method='PUT'"
								class="me-2"
							>Edit</a>
							<a href="#" v-on:click.prevent="this.deleteSkill(skill.id)">Delete</a>
						</td>
					</tr>
				</tbody>
			</table>
		</transition>
	</div>
</template>

<script>
	import VModal from '../components/VModal.vue'
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
						this.$router.go()
					}
				})    
				.catch((errors) => {
					console.log(errors);
				})
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