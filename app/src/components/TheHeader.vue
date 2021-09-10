<template>
	<nav class="navbar navbar-expand-sm shadow mb-5" :class='$colorScheme.navbar'>
		<div class="container-fluid">
			<router-link :to='{ name: "home" }' class="navbar-brand hula-logo">
				<svg height="50" viewBox="0 0 690 300" version="1.1" xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink" xml:space="preserve" xmlns:serif="http://www.serif.com/" style="fill-rule:evenodd;clip-rule:evenodd;stroke-linecap:round;stroke-linejoin:round;stroke-miterlimit:1.5;">
					<g transform="matrix(0.862694,0,0,0.862694,-203.255,-257.425)">
						<circle cx="384.358" cy="446.514" r="44.523"/>
					</g>
					<g transform="matrix(1.75318,0,0,1.75318,-570.459,-628.549)">
						<path id="hoop" d="M384.358,398.284C357.739,398.284 336.127,419.895 336.127,446.514C336.127,473.133 357.739,494.745 384.358,494.745C410.977,494.745 432.588,473.133 432.588,446.514C432.588,419.895 410.977,398.284 384.358,398.284ZM384.358,405.699C406.884,405.699 425.173,423.988 425.173,446.514C425.173,469.041 406.884,487.33 384.358,487.33C361.831,487.33 343.542,469.041 343.542,446.514C343.542,423.988 361.831,405.699 384.358,405.699Z"/>
					</g>
					<g transform="matrix(245.644,0,0,245.644,248.67,236.807)">
						<path d="M0.496,-0.314L0.496,-0.026C0.496,-0.008 0.511,0.008 0.53,0.008C0.549,0.008 0.564,-0.008 0.564,-0.026L0.564,-0.647C0.564,-0.665 0.549,-0.681 0.53,-0.681C0.511,-0.681 0.496,-0.665 0.496,-0.647L0.496,-0.376L0.148,-0.376L0.148,-0.647C0.148,-0.665 0.133,-0.681 0.114,-0.681C0.095,-0.681 0.08,-0.665 0.08,-0.647L0.08,-0.026C0.08,-0.008 0.095,0.008 0.114,0.008C0.133,0.008 0.148,-0.008 0.148,-0.026L0.148,-0.314L0.496,-0.314Z" style="fill-rule:nonzero;"/>
					</g>
					<g transform="matrix(245.644,0,0,245.644,406.864,236.807)">
						<path d="M0.087,-0.462C0.069,-0.462 0.055,-0.448 0.055,-0.43L0.055,-0.169C0.055,-0.074 0.087,0.013 0.241,0.013C0.396,0.013 0.427,-0.074 0.427,-0.169L0.427,-0.43C0.427,-0.448 0.413,-0.462 0.396,-0.462C0.378,-0.462 0.363,-0.448 0.363,-0.43L0.363,-0.169C0.363,-0.104 0.342,-0.048 0.241,-0.048C0.14,-0.048 0.119,-0.104 0.119,-0.169L0.119,-0.43C0.119,-0.448 0.104,-0.462 0.087,-0.462Z" style="fill-rule:nonzero;"/>
					</g>
					<g transform="matrix(245.644,0,0,245.644,525.265,236.807)">
						<path d="M0.097,-0.712C0.08,-0.712 0.065,-0.697 0.065,-0.68L0.065,-0.109C0.065,-0.032 0.102,0.008 0.169,0.008C0.185,0.008 0.199,-0.005 0.199,-0.022C0.199,-0.039 0.185,-0.052 0.169,-0.052C0.142,-0.052 0.129,-0.069 0.129,-0.109L0.129,-0.68C0.129,-0.697 0.115,-0.712 0.097,-0.712Z" style="fill-rule:nonzero;"/>
					</g>
					<g transform="matrix(245.644,0,0,245.644,586.43,236.807)">
						<path d="M0.232,-0.408C0.305,-0.408 0.35,-0.366 0.35,-0.298L0.35,-0.271L0.209,-0.271C0.101,-0.271 0.034,-0.218 0.034,-0.13C0.034,-0.038 0.101,0.013 0.207,0.013C0.267,0.013 0.325,-0.012 0.35,-0.059L0.35,-0.023C0.35,-0.006 0.364,0.008 0.381,0.008C0.398,0.008 0.412,-0.006 0.412,-0.023L0.412,-0.298C0.412,-0.4 0.348,-0.468 0.232,-0.468C0.148,-0.468 0.094,-0.433 0.066,-0.389C0.063,-0.384 0.061,-0.379 0.061,-0.372C0.061,-0.356 0.074,-0.343 0.09,-0.343C0.099,-0.343 0.106,-0.346 0.116,-0.357C0.147,-0.392 0.179,-0.408 0.232,-0.408ZM0.213,-0.045C0.135,-0.045 0.099,-0.078 0.099,-0.13C0.099,-0.188 0.144,-0.215 0.209,-0.215L0.35,-0.215L0.35,-0.173C0.35,-0.088 0.283,-0.045 0.213,-0.045Z" style="fill-rule:nonzero;"/>
					</g>
				</svg>
			</router-link>
			<ul class="navbar-nav">
				<li class="nav-item dropdown">
					<button id="usermenu" class='nav-link btn btn-unstyled fs-2 rounded-circle' data-bs-toggle="dropdown" aria-expanded="false">
						<i class='bi-person-circle'></i>
					</button>
					<ul v-if="loggedUser" class="dropdown-menu dropdown-menu-end" aria-labelledby="usermenu">
						<li v-if="loggedUser.isadmin === true">
							<router-link :to='{ name: "admin-projects", params: { id: loggedUser.id } }' class="dropdown-item">Admin</router-link>
						</li>
						<li>
							<router-link :to='{ name: "user", params: { id: loggedUser.id } }' class="dropdown-item">Profile</router-link>
						</li>
						<li>
							<button v-on:click="logOut()" class="dropdown-item">Log out</button>
						</li>
					</ul>
				</li>
			</ul>
		</div>
	</nav>
</template>

<script>
export default {
	name: 'TheHeader',
	data() {
		return {
			loggedUser: this.$store.state.loggeduser // Here loggedUser will result to menu rendering vs. not if you use the state directly
		}
	},

	methods: {
		async logOut() {
			const success = await this.$store.dispatch('logout')

			if (success) this.$flashMessage.show({
				type: 'success',
				title: 'Successfully logged out',
				time: 5000,
			})
		},
	}
};
</script>