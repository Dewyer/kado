import { Route, Switch, Redirect } from "react-router-dom";
import { GLOBAL_ROUTES } from "./routingConstants";
import { HomePage } from "src/components/pages/HomePage/HomePage";
import { LoginPage } from "src/components/pages/LoginPage/LoginPage";
import {useSelector} from "react-redux";
import {makeSelectUserInitiallyLoaded} from "src/store/selectors/global.selectors";
import {ProtectedRoute} from "src/components/pages/ProtectedRoute";
import {TeamPage} from "src/components/pages/TeamPage/TeamPage";
import {ApiGuidePage} from "src/components/pages/ApiGuidePage/ApiGuidePage";
import {ProblemsPage} from "src/components/pages/ProblemsPage/ProblemsPage";
import {ProblemDetailsPage} from "src/components/pages/ProblemDetailsPage/ProblemDetailsPage";
import {IndividualLeaderboardPage} from "src/components/pages/IndividualLeaderboardPage/IndividualLeaderboardPage";
import {TeamLeaderboardPage} from "src/components/pages/TeamLeaderboardPage/TeamLeaderboardPage";
import {TermsOfUseAndPrivacyPolicy} from "src/components/pages/TermsOfUseAndPrivacyPolicy/TermsOfUseAndPrivacyPolicy";
import {OauthCallbackPage} from "src/components/pages/OauthCallbackPage/OauthCallbackPage";
import {ReportProblem} from "src/components/pages/ReportProblem/ReportProblem";

const Routes = () => {
	const userLoaded = useSelector(makeSelectUserInitiallyLoaded());

	return !userLoaded ? <p>Loading ...</p> : (
		<Switch>
			<Route exact path={GLOBAL_ROUTES.HOME} component={HomePage} />
			<Route exact path={GLOBAL_ROUTES.LOGIN} component={LoginPage} />
			<Route exact path={GLOBAL_ROUTES.OAUTH_CALLBACK} component={OauthCallbackPage} />
			<Route exact path={GLOBAL_ROUTES.REPORT_PROBLEM} component={ReportProblem} />

			<Route exact path={GLOBAL_ROUTES.TERMS_OF_USE_AND_PRIVACY_POLICY} component={TermsOfUseAndPrivacyPolicy} />
			<ProtectedRoute exact path={GLOBAL_ROUTES.API_GUIDE} component={ApiGuidePage} />
			<ProtectedRoute exact path={GLOBAL_ROUTES.PROBLEMS} component={ProblemsPage} />
			<ProtectedRoute exact path={GLOBAL_ROUTES.TEAM} component={TeamPage} />
			<ProtectedRoute exact path={GLOBAL_ROUTES.INDIVIDUAL_LEADERBOARD} component={IndividualLeaderboardPage} />
			<ProtectedRoute exact path={GLOBAL_ROUTES.TEAM_LEADERBOARD} component={TeamLeaderboardPage} />
			<ProtectedRoute path={`${GLOBAL_ROUTES.PROBLEM_DETAILS}/:codeName`} component={ProblemDetailsPage} />

			<Redirect to="/" />
		</Switch>
	);
};

export default Routes;
