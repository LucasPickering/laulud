import React from "react";
import { Redirect, Route, Switch } from "react-router-dom";

import HomePage from "pages/Home/HomePage";
import NotFoundPage from "pages/NotFound/NotFoundPage";
import PageContainer from "./PageContainer";
import LoginPage from "pages/Login/LoginPage";
import TagsPage from "pages/Tags/TagsPage";
import SearchPage from "pages/Search/SearchPage";
import useAuthState from "hooks/useAuthState";
import { UserContext } from "util/UserContext";
import Loading from "components/Loading";

/**
 * Main component that handles global state fetching and rendering the page
 * container based on that
 */
const CoreContent: React.FC = () => {
  const authState = useAuthState();

  // TODO figure out how to use React.Suspense here instead
  if (authState === "loading") {
    return <Loading size="8rem" />;
  }

  const isLoggedIn = authState;
  return (
    <UserContext.Provider value={{ isLoggedIn }}>
      <PageContainer showHeader={isLoggedIn}>
        {isLoggedIn ? (
          <Switch>
            <Redirect from="/login" to="/" exact />

            <Route path="/" exact>
              <HomePage />
            </Route>
            <Route path="/search/:selectedUri?" exact>
              <SearchPage />
            </Route>
            <Route path="/tags/:selectedTag?" exact>
              <TagsPage />
            </Route>

            {/* Fallback route */}
            <Route>
              <NotFoundPage />
            </Route>
          </Switch>
        ) : (
          // User is not logged in - redirect everything to the login page
          <Switch>
            <Route path="/login" exact>
              <LoginPage />
            </Route>

            <Route
              path="*"
              render={({ location }) => {
                // Include a param on the login page to tell the server where
                // to send us back to after login
                const currentLocation = `${location.pathname}${location.search}${location.hash}`;
                const search =
                  currentLocation === "/"
                    ? ""
                    : `?next=${encodeURIComponent(currentLocation)}`;

                return (
                  <Redirect
                    to={{
                      pathname: "/login",
                      search,
                    }}
                  />
                );
              }}
            />
          </Switch>
        )}
      </PageContainer>
    </UserContext.Provider>
  );
};

export default CoreContent;
