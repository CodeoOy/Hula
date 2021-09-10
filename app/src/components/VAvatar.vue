<template>
	<div class="hula-avatar position-relative" :style="avatarStyle">
        {{ initials }}
        <i v-if='favorite' class='bi-star-fill text-yellow position-absolute bottom-0 end-0 mb-n2 me-n1'></i>
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
        user_id: '',
        firstname: '',
        lastname: '',
        size: Number,
        favorite: {
            type: Boolean,
            default: false,
        },
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
                lineHeight: '1.5rem',
                textAlign: 'center',
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
            var colorhex = this.user_id.substring(1,7)
            var r = parseInt(colorhex.substr(0, 2), 16);
            var g = parseInt(colorhex.substr(2, 2), 16);
            var b = parseInt(colorhex.substr(4, 2), 16);
            this.r = r < 166 ? r + 40 : r - 20
            this.g = g < 13 ? g : g - (g / 2)
            this.b = b < 112 ? b + 40 : b - 20
            return `rgba(${this.r}, ${this.g}, ${this.b}, 0.9)`
        },
    }
};
</script>