// sources:
// * https://css-tricks.com/styling-based-on-scroll-position/
// * https://codepen.io/rikschennink/pen/yZYbwQ

// The debounce function receives our function as a parameter
export function debounce<FP extends Array<unknown>>(fn: (...p: FP) => void) {
	// This holds the requestAnimationFrame reference, so we can cancel it if we wish
	let frame: number;

	// The debounce function returns a new function that can receive a variable number of arguments
	return (...params: FP) => {
		// If the frame variable has been defined, clear it now, and queue for next frame
		if (frame) {
			cancelAnimationFrame(frame);
		}

		// Queue our function call for the next frame
		frame = requestAnimationFrame(() => {
			// Call our function and pass any params we received
			fn(...params);
		});
	};
}
