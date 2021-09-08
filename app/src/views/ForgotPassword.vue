<template>
	<div class="container">
		<div class="row mt-4">
			<div class="col">
				<div class="p-3 rounded shadow bg-dark text-light">
					<FormForgotPassword v-if="showForm" v-on:success="showForm = false, resetSent = true"/>
                    <h2 v-if="resetSent">Password reset requested successfully, check your email.</h2>
				</div>
			</div>
		</div>
	</div>
</template>

<script>
import FormForgotPassword from '../forms/FormForgotPassword.vue'
export default {
    name: 'PageForgotPassword',
    data () {
        return {
            showForm: true,
            resetSent: false,
        }
    },
    components: {
        FormForgotPassword,
    },
}
</script>