import React from "react";

export const ScrollContext = React.createContext({
	scrollY: 0,
	setScrollY: (_: number) => {
		console.error("Not implemented!");
	},
});
