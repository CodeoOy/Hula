import state from '@store/state.js'
import { defineRule, configure } from 'vee-validate'
import { required, email, confirmed, min_value } from '@vee-validate/rules'

configure({
	validateOnBlur: false, // Empty required fields make the forms grow -> Forgot password link etc. will dodge the click
})

defineRule('required', required)
defineRule('email', email)
defineRule('confirmed', confirmed)
defineRule('min_value', min_value)

defineRule('date', value => {
	if (!value) return true
	const date = new Date(value)
	return date.toString() !== 'Invalid Date'
})

defineRule('afterDate', (value, [target], ctx) => {
	if (!value) return true
	const date = new Date(value)
	const min = new Date(ctx.form[target])
	if (date >= min) return true
	return `${ctx.field} should be after ${ctx.form[target]}`
})

defineRule('requiredNonAdmin', value => {
	return !!((state.loggeduser && state.loggeduser.isadmin) || value)
})
