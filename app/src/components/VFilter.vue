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
	function debounce(func, timeout = 300){
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
				console.log('filtering')

				let words = this.selection
					.toUpperCase()
					.trim()
					.replace(/ +/, ' ')

				words = this.singleWords ? words.split(' ') : [words]

				const props = Array.isArray(this.props) ? this.props : [this.props]

				const matches = this.items.filter(item => {
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

				this.$emit('filter', { keywords: words.filter(word => word), matches })
			}
		}
	}
</script>
