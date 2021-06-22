<template>
	<div>
        <table class="table table-dark table-striped text-light">
			<thead>
				<tr>
					<th></th>
					<th scope="col">{{ match.projectname }}</th>
					<th scope="col">{{ match.firstname }} {{ match.lastname }}</th>
				</tr>
			</thead>
			<tbody>
				<tr>
					<td>{{ match.skillname }}</td>
					<td>{{ match.required_minyears }}</td>
					<td>{{ match.user_years }}</td>
				</tr>
			</tbody>
		</table>
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