<template>
	<div>
		<VModal :modalTitle="formTitle" :modalID="'Skills'" v-on:updated-modal="chosenForm = '', chosenCategory = {}">
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
					v-on:click="formTitle = 'Add skill', chosenForm = 'CreateSkill', chosenSkill = {}, url='/api/skills', method='POST'"
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
										v-on:click="formTitle=`Add skill to ${category.label}`, chosenForm = 'CreateSkill', chosenCategory = category, chosenSkill = {}, url='/api/skills', method='POST'"
									><i class="bi-plus-circle-fill me-2"></i></a>
									<a 
										href="#"
										data-bs-toggle="modal"
										data-bs-target="#hulaModalSkills"
										v-on:click="formTitle='Edit category', chosenForm = 'Category', chosenCategory = category, url=`/api/skills/categories/${category.id}`, method='PUT'"
									><i class="bi-pencil-fill me-2"></i></a>
									<a
										href="#"
										data-bs-toggle="modal"
										data-bs-target="#hulaModalSkills" 
										v-on:click="formTitle = `Delete ${category.label}?`, chosenForm = 'Delete', url = `/api/skills/categories/${category.id}`, method = 'DELETE'"
									><i class="bi-trash-fill me-2"></i></a>
								</div>
							</div>
						</td>
						<td class="hoverable-td">
							<div class="title-actions" v-for="skill in filterSkills(category.id)" :key="skill" :value="skill.id">
								<span><span class="title-actions__maintitle">{{ skill.label }}</span><span class="title-actions__maintitle--dimmed"> ({{ getSkillScopeLabel(skill.skillscope_id) }})</span></span>
								<div class="title-actions__actions">
									<a 
										href="#"
										data-bs-toggle="modal"
										data-bs-target="#hulaModalSkills"
										v-on:click="formTitle=skill.label, chosenForm = 'CreateSkill', chosenSkill = skill, url=`/api/skills/${skill.id}`, method='PUT'"
									><i class="bi-pencil-fill me-2"></i></a>
									<a
										href="#"
										data-bs-toggle="modal"
										data-bs-target="#hulaModalSkills" 
										v-on:click="formTitle = `Delete ${skill.label}?`, chosenForm = 'Delete', url = `/api/skills/${skill.id}`, method = 'DELETE'"
									><i class="bi-trash-fill me-2"></i></a>
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
	import FormSkill from '../forms/FormSkill.vue'
	import FormSkillCategory from '../forms/FormSkillCategory.vue'
	import FormConfirmAction from '../forms/FormConfirmAction.vue'
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
				skills: [],
				categories: [],
				skillScopes: [],
			}
		},
		components: {
			VModal,
			FormSkill,
			FormSkillCategory,
			FormConfirmAction
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
			filterSkills(id) {
				return this.skills.filter(skill => skill.skillcategory_id == id)
			},
			getSkillScopeLabel(id) {
				if (id && this.skillScopes.length) {
					var scope = this.skillScopes.find(skillScope => skillScope.id == id)
					return scope.label
				}
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
					Delete: FormConfirmAction
				}
				return components[this.chosenForm]
			}
		},
		mounted() {
			this.getSkillCategories()
			this.getAllSkills()
			this.getAllScopes()
		}
	}
</script>