import { ReactElement } from "react";

import { SVGProps } from "../typings/commonTypes";

export default function HomeSVG(props: SVGProps): ReactElement {
	return (
		<svg
			aria-hidden="true"
			focusable="false"
			data-prefix="fad"
			data-icon="home"
			role="img"
			xmlns="http://www.w3.org/2000/svg"
			viewBox="0 0 576 512"
			className="svg-inline--fa fa-home fa-w-18 fa-3x"
			{...props}
		>
			<g className="fa-group">
				<path
					fill="currentColor"
					d="M336 463.59V368a16 16 0 0 0-16-16h-64a16 16 0 0 0-16 16v95.71a16 16 0 0 1-15.92 16L112 480a16 16 0 0 1-16-16V300.06l184.39-151.85a12.19 12.19 0 0 1 15.3 0L480 300v164a16 16 0 0 1-16 16l-112-.31a16 16 0 0 1-16-16.1z"
					className="fa-secondary"
				></path>
				<path
					fill="currentColor"
					d="M573.32 268.35l-25.5 31a12 12 0 0 1-16.9 1.65L295.69 107.21a12.19 12.19 0 0 0-15.3 0L45.17 301a12 12 0 0 1-16.89-1.65l-25.5-31a12 12 0 0 1 1.61-16.89L257.49 43a48 48 0 0 1 61 0L408 116.61V44a12 12 0 0 1 12-12h56a12 12 0 0 1 12 12v138.51l83.6 68.91a12 12 0 0 1 1.72 16.93z"
					className="fa-primary"
				></path>
			</g>
		</svg>
	);
}
