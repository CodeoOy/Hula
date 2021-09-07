<template>
	<div>
		<VFilter
			:items="items"
			:placeholder='placeholder'
			:props='props'
			:singleWords='false'
			@filter="onFilter"
			@focusin='focused = true'
			@focusout='focused = false'
			@keydown.up.prevent='selected--'
			@keydown.down.prevent='selected++'
			@keypress.enter='select'
		/>
		<ul class='list-unstyled mt-2 mb-0'>
			<li v-for='(item, index) in filteredItems' :key='item.id || index' :class='{ "active": isSelected(index) }'>
				<button class='btn btn-unstyled px-2 py-1' @click='onSelect(item)'>{{ item.name }}</button>
			</li>
		</ul>
	</div>
</template>

<script>
	import VFilter from './VFilter.vue'

	export default {
		name: 'VFilterList',

		components: {
			VFilter,
		},

		props: {
			items: {
				type: Array,
				required: true,
			},
			props: {
				type: [String, Array],
				default: 'name',
			},
			placeholder: String,
		},

		data() {
			return {
				filteredItems: this.items,
				selected: 0,
				focused: false,
			}
		},

		watch: {
			filteredItems(value, oldValue) {
				const item = oldValue[this.selected]
				const index = value.indexOf(item) 
				if (index > -1) this.selected = index
				else if (this.selected >= value.length) this.selected = value.length - 1
			},

			selected(value) {
				this.selected = Math.max(0, Math.min(value, this.filteredItems.length - 1))
			},

			focused(value) {
				if (!value) this.selected = 0
			},
		},

		methods: {
			onFilter(items) {
				this.filteredItems = items
			},

			isSelected(index) {
				return this.focused && this.selected == index
			},

			select() {
				const item = this.filteredItems[this.selected]
				this.onSelect(item)
			},

			onSelect(item) {
				this.$emit('select', item)
			},
		},
	}
</script>

<style scoped lang='scss'>
	.active {
		background-color: var(--bs-primary);

		& > .btn {
			color: var(--bs-white);
		}
	}
</style>