<template>
	<div class="container mt-4">
		<div class="row gx-4">
			<div class="col-md-4">
                <div>Project: {{ $route.params.id }}</div>
			</div>
			<div class="col-md-8">
				<ul class="nav nav-tabs nav-dark">
					<li class="nav-item">
						<a class="nav-link" v-bind:class="{ 'active': tab == 1, '': tab != 1 }" @click="tab = 1" href="#">Tab 1</a>
					</li>
					<li class="nav-item">
						<a class="nav-link" v-bind:class="{ 'active': tab == 2, '': tab != 2 }" @click="tab = 2" href="#">Tab 2</a>
					</li>
					<li class="nav-item">
						<a class="nav-link" v-bind:class="{ 'active': tab == 3, '': tab != 3 }" @click="tab = 3" href="#">Tab 3</a>
					</li>
				</ul>
			</div>
		</div>
	</div>
</template>

<script>
	export default {
		name: 'Project',
		data() {
			return {
				tab: 1,
			}
		},
	}
</script>