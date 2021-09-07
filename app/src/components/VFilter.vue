<template>
	<input
		v-model='selection'
		type='text'
		:placeholder='placeholder'
		@input = 'change'
		class='form-control'
	/>
</template>

<script>
	export default {
		name: 'VFilter',

		props: {
			items: {
				type: Array,
				required: true
			},
			props: {
				type: [String, Array],
				default: 'name',
			},
			singleWords: {
				type: Boolean,
				default: true,
			},
			placeholder: String,
		},

		data() {
			return {
				selection: '',
			}
		},

		methods: {
			change() {
				const matches = this.items.filter(item => {
					let words = this.selection
						.toUpperCase()
						.trim()
						.replace(/ +/, ' ')

					words = this.singleWords ? words.split(' ') : [words]

					const props = Array.isArray(this.props) ? this.props : [this.props]
					for (let prop of props) {
						const values = Array.isArray(item[prop]) ? item[prop] : [item[prop]]
						for (let value of values) {
							value = String(value).toUpperCase()
							for (const word of words) {
								if (value.includes(word)) return true
							}
						}
					}
				})

				this.$emit('filter', matches)
			},
		}
	}
</script>