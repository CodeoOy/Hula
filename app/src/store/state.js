export default {
	colorScheme: getComputedStyle(document.documentElement).getPropertyValue('--color-scheme').trim(),
	loggeduser: JSON.parse(localStorage.getItem('user')),
	chosenproject: null,
	cookieConsent: JSON.parse(localStorage.getItem('cookieConsent')),
	projects: [],
	skillCategories: [],
	skillScopes: [],
	skills: [],
	skillLevels: [],
}