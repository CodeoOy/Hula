<template>
	<div class="container">
		<ul v-if='showNavigation' ref='navigation' class="nav nav-tabs flex-nowrap overflow-auto">
			<li v-for='{ name, label } of navigation' :key='name' class='nav-item text-nowrap'>
				<router-link :to='{ name }' class='nav-link'>{{ label }}</router-link>
			</li>
		</ul>
		<router-view v-slot='{ Component }'>
			<keep-alive>
				<component :is='Component' />
			</keep-alive>
		</router-view>
	</div>
</template>

<script>
	export const navigation = [
		{ name: 'admin-projects', label: 'Projects' },
		{ name: 'admin-users', label: 'Users' },
		{ name: 'admin-skills', label: 'Skills & Categories' },
		{ name: 'admin-scopes', label: 'Scopes & Levels' },
		{ name: 'admin-offers', label: 'Offers' },
	]

	export default {
		name: 'Admin',
		
		data() {
			return {
				navigation,
				showNavigation: true,
			}
		},

		mounted() {
			const navigationWidth = this.$refs.navigation.scrollWidth

			this.resizeObserver = new ResizeObserver(([entry]) => {
				window.requestAnimationFrame(() => {
					this.showNavigation = entry.contentRect.width >= navigationWidth
				})
			})

			this.resizeObserver.observe(this.$el)
		},

		beforeUnmount() {
			this.resizeObserver.unobserve(this.$el)
		},
	}
</script>
