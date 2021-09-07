import 'semantic-ui-less/semantic.less';

import React from "react";
import ReactDOM from "react-dom";
import { Router } from "react-router-dom";
import { Provider } from "react-redux";
import "./styles/index.scss";
import App from "./App";
import reportWebVitals from "./reportWebVitals";
import { store } from "./store";
import "./polyfills/array-polyfill";
import { history } from "./helpers/history";


ReactDOM.render(
	<React.StrictMode>
		<Provider store={store}>
			<Router history={history}>
				<App />
			</Router>
		</Provider>
	</React.StrictMode>,
	document.getElementById("root")
);

reportWebVitals();

// https://github.com/Semantic-Org/example-github/blob/master/semantic/src/themes/github/globals/site.variables
// https://semantic-ui.com/usage/theming.html