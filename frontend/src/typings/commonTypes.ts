import React from "react";

// https://stackoverflow.com/questions/62715379/how-to-declare-types-of-props-of-svg-component-react-typescript-and-webpack
export type SVGProps = React.SVGProps<SVGSVGElement>;

export type IdType = string | number;

export interface Identifiable {
	id?: string | number;
}
