<template>
	<div class="container">
		<div class="row mt-4">
			<div class="col">
				<div class="p-3 rounded-2 content-box bg-dark text-light">
					<p>
						<button @click="show = !show" class="btn btn-gradient">
							Transition
						</button>
					</p>
					<p>
						<button @click="logoutUser()" class="btn btn-gradient">
							Log out
						</button>
					</p>
					<transition name="fadeHeight">
						<p v-if="show">hello</p>
					</transition>
					<p>{{ message }}</p>
				</div>
			</div>
		</div>
	</div>
</template>

<script>
	export default {
		name: 'Dashboard',
		data() {
			return {
				message: "Dashboard",
				show: false
			}
		}
	}
</script>