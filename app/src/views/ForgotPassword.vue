<template>
	<div class="container">
		<div class="row mt-4">
			<div class="col">
				<div class="p-3 rounded-2 content-box bg-dark text-light">
					<FormForgotPassword v-if="showForm" />
				</div>
			</div>
		</div>
	</div>
</template>

<script>
import FormForgotPassword from '../forms/FormForgotPassword.vue'
export default {
    name: 'PageForgotPassword',
    components: {
        FormForgotPassword,
    },
    computed: {
        title () {
            if (this.$store.state.errorObject) {
                return this.$store.state.errorObject.title
            } else {
                return "Page not found."
            }
        },
        message() {
            if (this.$store.state.errorObject) {
                return this.$store.state.errorObject.title
            } else {
                return "Try the navigation."
            }
        }
    }	
}
</script>