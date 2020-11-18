import React from "react";
import { CssBaseline } from "@material-ui/core";
import { ThemeProvider } from "@material-ui/styles";
import { BrowserRouter, Route, Switch } from "react-router-dom";
import { QueryCache, ReactQueryCacheProvider } from "react-query";

import HomePage from "pages/Home/HomePage";
import NotFoundPage from "pages/NotFound/NotFoundPage";
import PageContainer from "./components/generic/PageContainer";
import theme from "./theme";

const queryCache = new QueryCache();

const App: React.FC = () => {
  return (
    <ReactQueryCacheProvider queryCache={queryCache}>
      <ThemeProvider theme={theme}>
        <CssBaseline />
        <BrowserRouter>
          <PageContainer>
            <Switch>
              <Route path="/" exact>
                <HomePage />
              </Route>

              {/* Fallback route */}
              <Route>
                <NotFoundPage />
              </Route>
            </Switch>
          </PageContainer>
        </BrowserRouter>
      </ThemeProvider>
    </ReactQueryCacheProvider>
  );
};

export default App;
