<template>
<div class="autocomplete me-2 w-100">
    <input class="form-control" type="text" v-model="selection" :placeholder="placeholder"
        @keydown.enter = 'enter'
        @keydown.down = 'down'
        @keydown.up = 'up'
        @input = 'change'
    />
    <ul class="dropdown-menu" style="width:100%" v-bind:class="{'show':openSuggestion}">
        <li v-for="(suggestion, index) in matches"
            v-bind:class="{'active': isActive(index)}"
            @click="suggestionClick(suggestion)"
            v-bind:key="suggestion.id"
        >
            {{ suggestion.name }}
        </li>
    </ul>
</div>
</template>
<script>
    export default {
        name: 'AutoComplete',
        data() {
            return {
                open: false,
                current: 0
            }
        },
        props: {
            suggestions: {
                type: Array,
                required: true
            },
            selection: {
                type: String,
                required: true,
                twoWay: true
            },
			placeholder: String,
            dropdown: Boolean,
            filterProperties: '',
        },
        computed: {
            matches() {
                if (!this.suggestions.length) {
                    return []
                }
                var matches = this.suggestions.filter(item => {
                    return item[this.filterProperties].toUpperCase().indexOf(this.selection.toUpperCase()) >= 0;
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
                this.$emit('autoComplete', this.matches[this.current]);
                this.open = false;
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
                this.$emit('autoComplete', project)
                //this.selection = this.matches[index];
                this.open = false;
            },
        }
    }
</script>