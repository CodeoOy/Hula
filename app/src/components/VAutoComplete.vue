<template>
    <div class='position-relative'>
        <div class="input-group w-100">
            <input class="form-control" type="text" v-model="selection" :placeholder="placeholder"
                @keydown.enter = 'enter'
                @keydown.down = 'down'
                @keydown.up = 'up'
                @input = 'change'
            />
            <slot name='button' />
        </div>
        <ul class="dropdown-menu" style="width:100%" v-bind:class="{'show':openSuggestion}">
            <li v-for="(suggestion, index) in matches" :key="suggestion.id">
                <a href='#'
                    @click="suggestionClick(suggestion)"
                    class='dropdown-item'
                    :class="{'active': isActive(index)}"
                >
                    {{ suggestion[dropdownLabel] }}
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
                type: String,
                default: 'name'
            },
            filterProperties: '',
        },
        computed: {
            matches() {
                var matches = this.suggestions.filter(item => {
                    const selection = this.selection.toUpperCase()
                    const props = Array.isArray(this.filterProperties) ? this.filterProperties : [this.filterProperties]
                    for (const prop of props) {
                        if (String(item[prop]).toUpperCase().includes(selection)) return true
                    }
                });
                if (!this.dropdown) {
                    this.$emit('autoComplete', matches)
                }
                return matches;
            },
            openSuggestion() {
                if (this.dropdown) {
                    return this.selection !== "" && this.matches.length != 0 && this.open === true;
                } else {
                    return false
                }
            }
        },
        methods: {
            enter() {
                if (this.dropdown && this.selection.length) {
                    this.$emit('autoComplete', this.matches[this.current]);
                    this.open = false;
                    this.selection = this.matches[this.current][this.dropdownLabel]
                }
            },
            up() {
                if(this.current > 0)
                    this.current--;
            },
            down() {
                if(this.current < this.matches.length - 1)
                    this.current++;
            },
            isActive(index) {
                return index === this.current;
            },
            change() {
                if (this.open == false) {
                    this.open = true;
                    this.current = 0;
                }
            },
            suggestionClick(project) {
                if (this.dropdown) {
                    this.selection = project[this.dropdownLabel]
                    this.$emit('autoComplete', project)
                    this.open = false;
                }
            },
        }
    }
</script>
