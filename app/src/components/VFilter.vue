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
	function debounce(func, timeout = 250){
		let timer
		return (...args) => {
			clearTimeout(timer)
			timer = setTimeout(() => {
				func.apply(this, args)
			}, timeout)
		}
	}

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

		watch: {
			items() {
				this.filter()
			}
		},

		mounted() {
			this.change = debounce(this.filter)
		},

		methods: {
			filter() {
				let keywords = this.selection
					.trim()
					.replace(/ +/, ' ')

				keywords = this.singleWords
					? keywords.split(' ')
					: [keywords]

				keywords = keywords.filter(word => word)

				const props = Array.isArray(this.props)
					? this.props
					: [this.props]

				const pattern = keywords.length
					? new RegExp(keywords.map(word => word.replace(/[.*+?^${}()|[\]\\]/g, '\\$&')).join('|'), 'i')
					: null

				const matches = !keywords.length
					? this.items
					: this.items.filter(item => props.some(prop => {
						const values = Array.isArray(item[prop]) ? item[prop] : [item[prop]]
						return values.some(value => pattern.test(value))
					}))

				this.$emit('filter', {
					keywords,
					pattern,
					matches,
				})
			}
		}
	}
</script>
