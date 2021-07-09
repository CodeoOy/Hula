<template>
	<div>
		<VModal :modalTitle="formTitle" :modalID="'Skills'">
			<component 
				:is='modalComponent' 
				:chosenSkill="chosenSkill" 
				:chosenCategory="chosenCategory"
				:url="url"
				:method="method"
				v-on:form-sent="hideModalUpdate"
			/>
		</VModal>
		<div class="d-flex flex-row justify-content-between align-items-start">
			<h2>Skills</h2>
			<div>
				<button
					class="btn btn-gradient me-2"
					data-bs-toggle="modal"
					data-bs-target="#hulaModalSkills"
					v-on:click="formTitle = 'Add skill', chosenForm = 'CreateSkill', chosenSkill = chosenSkillDefault, url='/api/skills', method='POST'"
				>New skill</button>
				<button
					class="btn btn-gradient"
					data-bs-toggle="modal"
					data-bs-target="#hulaModalSkills"
					v-on:click="formTitle = 'Add category', chosenForm = 'Category', chosenCategory.parent_id = '', url='/api/skills/categories', method='POST'"
				>New category</button>
			</div>
		</div>
		<transition name="fadeHeight">
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
									<a 
										href="#"
										data-bs-toggle="modal"
										data-bs-target="#hulaModalSkills"
										v-on:click="formTitle=`Add skill to ${category.label}`, chosenForm = 'CreateSkill', chosenCategory = category, chosenSkill = null, url='/api/skills', method='POST'"
									><i class="bi-plus-circle-fill me-2"></i></a>
									<a 
										href="#"
										data-bs-toggle="modal"
										data-bs-target="#hulaModalSkills"
										v-on:click="formTitle='Edit category', chosenForm = 'Category', chosenCategory = category, url=`/api/skills/categories/${category.id}`, method='PUT'"
									><i class="bi-pencil-fill me-2"></i></a>
									<a href="#" v-on:click.prevent="this.deleteCategory(category.id)"><i class="bi-trash-fill me-2"></i></a>
								</div>
							</div>
						</td>
						<td class="hoverable-td">
							<div class="title-actions" v-for="skill in filterSkills(category.id)" :key="skill" :value="skill.id">
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
				chosenCategory: {},
				chosenSkillDefault: {
					label: '',
					skillcategory_id: null,
					skillscope_id: null,
				},
				skills: [],
				categories: [],
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
				.then(response => { 
					return (response.status >= 200 && response.status <= 299) ? response.json() : this.$store.commit('errorHandling', response)
				})
				.then(response => { 
					this.skills = response;
				})    
				.catch((errors) => {
					this.$store.commit('errorHandling', errors)
				})
			},
			getSkillCategories() {
				fetch('/api/skills/categories', {method: 'GET'})
				.then(response => {
					return (response.status >= 200 && response.status <= 299) ? response.json() : this.$store.commit('errorHandling', response)
				}) 
				.then(response => { 
					this.categories = response;
				})
			},
			deleteSkill(id) {
				fetch(`/api/skills/${id}`, {method: 'DELETE'})
				.then(response => { 
					return (response.status >= 200 && response.status <= 299) ? response : this.$store.commit('errorHandling', response)
				})
				.then(() => { 
					this.getAllSkills()
				})    
				.catch((errors) => {
					this.$store.commit('errorHandling', errors)
				})
			},
			deleteCategory(id) {
				fetch(`/api/skills/categories/${id}`, {method: 'DELETE'})
				.then(response => { 
					return (response.status >= 200 && response.status <= 299) ? response : this.$store.commit('errorHandling', response)
				})
				.then((response) => { 
					console.log(response)
					this.getSkillCategories()
				})
			},
			filterSkills(id) {
				return this.skills.filter(skill => skill.skillcategory_id == id)
			},
			hideModalUpdate() {
				this.getSkillCategories()
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
			this.getSkillCategories()
			this.getAllSkills()
		}
	}
</script>