<template>
<div class="autocomplete me-2">
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
            v-bind:key="suggestion"
        >
            <a :href="`/app/project/${suggestion.id}`">{{ suggestion.name }}</a>
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
        },
        computed: {
            matches() {
                var matches = this.suggestions.filter(project => {
                    return project.name.toUpperCase().indexOf(this.selection.toUpperCase()) >= 0;
                });
                console.log(matches)
                this.$emit('autoComplete', matches)
                return matches;
            },
            openSuggestion() {
                return this.selection !== "" &&
                    this.matches.length != 0 &&
                    this.open === true;
            }
        },
        methods: {
            enter() {
                this.selection = this.matches[this.current];
                this.open = false;
                console.log(this.selection)
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