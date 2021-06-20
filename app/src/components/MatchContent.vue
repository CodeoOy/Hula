<template>
	<div>
        {{ match }}
    </div>
</template>

<script>
	export default {
		name: 'MatchContent',
		data() {
			return {
				skills: {}
			}
		},
        props: {
            match: {}
        }
	}
</script>