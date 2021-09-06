import { defineRule } from 'vee-validate'
import { required, email, confirmed, min_value } from '@vee-validate/rules'

defineRule('required', required)
defineRule('email', email)
defineRule('confirmed', confirmed)
defineRule('min_value', min_value)
defineRule('date', value => {
	const date = new Date(value)
	return date.toString() !== 'Invalid Date'
})
defineRule('afterDate', (value, [target], ctx) => {
	const date = new Date(value)
	const min = new Date(ctx.form[target])
	if (date >= min) return true
	return `${ctx.field} should be after ${ctx.form[target]}`
})

defineRule('requiredNonAdmin', value => {
	return !!((store.state.loggeduser && store.state.loggeduser.isadmin) || value)
})
