<template>
	<div class='avatar position-relative' :style='{ "--avatar-bg-color": color }'>
		<div class='avatar-initials'>{{ initials }}</div>
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

			color() {
				const adjust = (value, angle, arc, factor) => {
					if (h > angle - arc / 2 && h < angle + arc / 2) {
						const offset = h - (angle - arc / 2)
						value*= 1 + Math.sin(offset / arc * Math.PI) * factor
					}
					return value
				}

				let hash = 0
				for (let i = 0; i < this.id.length; i++) {
					hash = this.id.charCodeAt(i) + ((hash << 5) - hash)
				}

				const styles = getComputedStyle(document.documentElement)

				const h = Math.abs(hash % 360)
				let s = parseFloat(styles.getPropertyValue('--avatar-bg-saturation'))
				let l = parseFloat(styles.getPropertyValue('--avatar-bg-lightness'))

				// Yellow
				l = adjust(l, 60, 120, -0.3)
				s = adjust(s, 60, 120, 0.25)

				// Green
				l = adjust(l, 120, 120, -0.3)
				s = adjust(s, 120, 120, 0.25)

				// Cyan
				l = adjust(l, 180, 120, -0.3)
				s = adjust(s, 180, 120, 0.25)

				// Blue
				l = adjust(l, 240, 120, 0.2)
				s = adjust(s, 240, 120, 0.5)


				l = Math.min(l, 100)
				s = Math.min(s, 100)

				return `hsl(${h}, ${s}%, ${l}%)`
			},
		},
	}
</script>
