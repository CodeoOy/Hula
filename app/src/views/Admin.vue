<template>
	<div class="container">
		<ul class="nav nav-tabs">
			<li v-for='{ name, label } of navigation' :key='name' class='nav-item'>
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
	export default {
		name: 'Admin',
		
		data() {
			return {
				navigation: [
					{ name: 'admin-projects', label: 'Projects' },
					{ name: 'admin-users', label: 'Users' },
					{ name: 'admin-skills', label: 'Skills & Categories' },
					{ name: 'admin-scopes', label: 'Scopes & Levels' },
					{ name: 'admin-offers', label: 'Offers' },
				],
			}
		},
	}
</script>
