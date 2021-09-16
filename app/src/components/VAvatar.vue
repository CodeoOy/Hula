<template>
	<div class='avatar position-relative' :style='avatarStyles'>
		<div class='avatar-initials' :class='initialClasses'>{{ initials }}</div>
		<i v-if='favorite' class='bi-star-fill text-yellow position-absolute bottom-0 end-0 mb-n2 me-n1'></i>
	</div>
</template>

<script>
	export default {
		name: 'VAvatar',

		props: {
			id: {
				type: String,
				required: true,
			},
			firstName: {
				type: String,
				required: true,
			},
			lastName: {
				type:  String,
				required: true,
			},
			favorite: {
				type: Boolean,
				default: false,
			},
		},

		computed: {
			initials() {
				return this.firstName[0] + this.lastName[0]
			},

			colors() {
				let hash = 1
				for (let i = 0; i < this.id.length; i++) {
					hash = this.id.charCodeAt(i) + ((hash << 5) - hash)
				}

				const styles = getComputedStyle(document.documentElement)
				const palette = styles.getPropertyValue('--avatar-palette').trim().split(',').map(color => color.trim().split('/'))

				const [bg, text] = palette[Math.abs(hash % palette.length)]

				return { bg, text }
			},

			avatarStyles() {
				return `--avatar-bg-color: ${this.colors.bg}`
			},

			initialClasses() {
				return this.colors.text == 'light'
					? 'avatar-initials-light'
					: 'avatar-initials-dark'
			},
		},
	}
</script>
