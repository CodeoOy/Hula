// Set widths of tr and td elements to prevent shrinking because of position: absolute
export const onBeforeTrLeave = el => {
	[el, ...el.children].forEach(el => el.style.width = `${el.offsetWidth}px`)
}
