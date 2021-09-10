<template>
	<div class="container mt-5">
		<div class="card shadow" :class='$colorScheme.card'>
            <div class='card-header'>
                <h1 class='h3 mb-0'>Request a password reset</h1>
            </div>
			<div class='card-body'>
				<FormForgotPassword v-if="showForm" v-on:success="showForm = false, resetSent = true"/>
				<h2 v-if="resetSent">Password reset requested successfully, check your email.</h2>
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