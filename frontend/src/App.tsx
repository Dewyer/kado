import { useState, useEffect } from "react";
import { useDispatch, useSelector } from "react-redux";

import Routes from "./routing/Routes";
import NavBar from "./components/layout/NavBar/NavBar";
import { ScrollContext } from "./contexts/scrollContext";
import { Helmet } from "react-helmet";
import "./App.scss";
import * as actions from "./store/actions/index";
import { RootState } from "./store";

function App() {
	const [scrollY, setScrollY] = useState(0);
	const dispatch = useDispatch();
	const { loading, error } = useSelector((state: RootState) => state.global);

	useEffect(() => {
		dispatch(actions.getAllData());
	}, []);

	return (
		<ScrollContext.Provider value={{ scrollY, setScrollY }}>
			<Helmet titleTemplate="%s - SnapVote" defaultTitle="SnapVote" />
			<NavBar />
			{!loading && <Routes />}
		</ScrollContext.Provider>
	);
}

export default App;
