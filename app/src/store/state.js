export default {
	loggeduser: JSON.parse(localStorage.getItem('user')),
	chosenproject: {},
	projects: JSON.parse(localStorage.getItem('projects')),
	nextpage: '',
	errorObject: null,
}