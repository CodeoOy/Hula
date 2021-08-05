<template>
	<div class="hula-avatar" :style="avatarStyle">
        {{ initials }}
	</div>
</template>

<script>
export default {
	name: 'VAvatar',
    data() {
        return {
            r: 0,
            g: 0,
            b: 0,
        }
    },
	props: {
        id: '',
        firstname: '',
        lastname: '',
        size: Number
	},
    computed: {
        initials() {
            return this.firstname[0] + this.lastname[0];
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
                color: this.textColor(),
            }
        },
    },
    methods: {
        // get color brightness from hex
        textColor() {
            var brightness = (this.r * 299 + this.g * 587 + this.b * 114) / 1000;
            return brightness < 128 ? '#97ffcb' : '#210e26';
        },
        color() {
            var colorhex = this.id.substring(1,7)
            var r = parseInt(colorhex.substr(1, 2), 16);
            var g = parseInt(colorhex.substr(3, 2), 16);
            var b = parseInt(colorhex.substr(5, 2), 16);
            this.r = (r + 166) / 2
            this.g = (g + 13) / 5
            this.b = (b + 112) / 2
            return `rgba(${this.r}, ${this.g}, ${this.b}, 0.9)`
        },
        /*
        colorHex() {
            var colorhex = this.id.substring(1,7)
            return '#' + colorhex
        }
        */
        // TODO: Tint the hue towards the theme colors
    }
};
</script>