<template>
    <div class='position-relative form-control bg-transparent p-0 border-0'>
        <input class='form-control form-control-inner' type='text' v-model='selection' :placeholder='placeholder'
            @keydown.enter = 'enter'
            @keydown.down = 'down'
            @keydown.up = 'up'
            @input = 'change'
        />
        <ul class='dropdown-menu' style='width:100%' v-bind:class='{ show: openSuggestion }'>
            <li v-for='(suggestion, index) in matches' :key='suggestion.id'>
                <a href='#'
                    @click='suggestionClick(suggestion)'
                    class='dropdown-item'
                    :class='{ active: isActive(index) }'
                >
                    {{ itemLabel(suggestion) }}
                </a>
            </li>
        </ul>
    </div>
</template>
<script>
    export default {
        name: 'VAutoComplete',
        data() {
            return {
                selection: '',
                open: false,
                current: 0
            }
        },
        props: {
            suggestions: {
                type: Array,
                required: true
            },
			placeholder: String,
            dropdown: Boolean,
            dropdownLabel: {
                type: [String, Function],
                default: 'name'
            },
            filterProperties: '',
            singleWords: {
                type: Boolean,
                default: true,
            },
        },
        computed: {
            matches() {
                var matches = this.suggestions.filter(item => {
                    let words = this.selection
                        .toUpperCase()
                        .trim()
                        .replace(/ +/, ' ')

                    words = this.singleWords ? words.split(' ') : [words]

                    const props = Array.isArray(this.filterProperties) ? this.filterProperties : [this.filterProperties]
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
                if (!this.dropdown) {
                    this.$emit('autoComplete', matches)
                }
                return matches
            },
            openSuggestion() {
                if (this.dropdown) {
                    return this.selection !== '' && this.matches.length != 0 && this.open === true
                } else {
                    return false
                }
            }
        },
        methods: {
            enter() {
                if (this.dropdown && this.selection.length && this.matches[this.current]) {
                    this.$emit('autoComplete', this.matches[this.current])
                    this.open = false
                    this.selection = this.itemLabel(this.matches[this.current])
                }
            },
            up() {
                if(this.current > 0)
                    this.current--
            },
            down() {
                if(this.current < this.matches.length - 1)
                    this.current++
            },
            isActive(index) {
                return index === this.current
            },
            change() {
                if (this.open == false) {
                    this.open = true
                    this.current = 0
                }
            },
            suggestionClick(item) {
                if (this.dropdown) {
                    this.selection = this.itemLabel(item)
                    this.$emit('autoComplete', item)
                    this.open = false
                }
            },
            itemLabel(item) {
                return typeof this.dropdownLabel == 'function'
                    ? this.dropdownLabel(item)
                    : item[this.dropdownLabel]
            },
        }
    }
</script>

<style lang='scss' scoped>
    .form-control-inner {
        border-radius: inherit;

        &:focus {
            position: relative;
            z-index: 3;
        }
    }
</style>