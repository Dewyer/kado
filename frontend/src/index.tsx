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
import { ToastContainer } from 'react-toastify';
import "react-toastify/dist/ReactToastify.css";
import {QueryClient, QueryClientProvider} from "react-query";

const queryClient = new QueryClient()

ReactDOM.render(
	<React.StrictMode>
		<Provider store={store}>
			<QueryClientProvider client={queryClient}>
				<ToastContainer />
				<Router history={history}>
					<App />
				</Router>
			</QueryClientProvider>
		</Provider>
	</React.StrictMode>,
	document.getElementById("root")
);

reportWebVitals();

// https://github.com/Semantic-Org/example-github/blob/master/semantic/src/themes/github/globals/site.variables
// https://semantic-ui.com/usage/theming.html