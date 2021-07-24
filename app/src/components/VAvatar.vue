<template>
	<div class="hula-avatar" :style="avatarStyle">
        {{ initials }}
	</div>
</template>

<script>
export default {
	name: 'VAvatar',
	props: {
        user: {},
        size: Number
	},
    computed: {
        initials() {
            return this.user.firstname[0] + this.user.lastname[0];
        },
        avatarStyle() {
            return {
                backgroundColor: this.color(),
                borderRadius: '50%',
                fontSize: '0.8rem',
                width: this.size || '24px',
                height: this.size || '24px',
                display: 'inline-block',
                marginRight: '5px',
                color: this.textColor(this.color()),
            }
        },
    },
    methods: {
        // get color brightness from hex
        textColor(color) {
            var r = parseInt(color.substr(2, 2), 16);
            var g = parseInt(color.substr(4, 2), 16);
            var b = parseInt(color.substr(6, 2), 16);
            var brightness = (r * 299 + g * 587 + b * 114) / 1000;
            return brightness < 128 ? '#fff' : '#000';
        },
        color() {
            var colorhex = this.user.id.substring(0,6)
            return '#' + colorhex
        }
    }
};
</script>