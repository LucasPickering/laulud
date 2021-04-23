import React from "react";
import { CircularProgress, makeStyles } from "@material-ui/core";
import { Redirect, Route, Switch } from "react-router-dom";

import HomePage from "pages/Home/HomePage";
import NotFoundPage from "pages/NotFound/NotFoundPage";
import PageContainer from "./PageContainer";
import LoginPage from "pages/Login/LoginPage";
import TagsPage from "pages/Tags/TagsPage";
import SearchPage from "pages/Search/SearchPage";
import useAuthState from "hooks/useAuthState";

const useStyles = makeStyles({
  loadingWrapper: {
    display: "flex",
    alignItems: "center",
    justifyContent: "center",
    width: "100%",
    height: "100%",
  },
});

const CoreContent: React.FC = () => {
  const classes = useStyles();
  const authState = useAuthState();

  if (authState === "loading") {
    return (
      <div className={classes.loadingWrapper}>
        <CircularProgress size="8rem" />
      </div>
    );
  }

  if (!authState) {
    // User is not logged in - redirect everything to the login page
    return (
      <PageContainer>
        <Switch>
          <Route path="/login" exact>
            <LoginPage />
          </Route>

          {/* TODO include ?next param here */}
          <Redirect from="/" to="/login" />
        </Switch>
      </PageContainer>
    );
  }

  // If we get this far, we know the user is logged in
  return (
    <PageContainer>
      <Switch>
        <Redirect from="/login" to="/" exact />

        <Route path="/" exact>
          <HomePage />
        </Route>
        <Route path="/search/:selectedUri?" exact>
          <SearchPage />
        </Route>
        <Route path="/tags/:tag?" exact>
          <TagsPage />
        </Route>

        {/* Fallback route */}
        <Route>
          <NotFoundPage />
        </Route>
      </Switch>
    </PageContainer>
  );
};

export default CoreContent;
