import { CssBaseline } from "@material-ui/core";
import { ThemeProvider } from "@material-ui/styles";
import React from "react";
import { BrowserRouter, Route, Switch } from "react-router-dom";

import PageContainer from "./PageContainer";
import MainPage from "./MainPage";
import theme from "../theme";

const App: React.FC = () => {
  return (
    <ThemeProvider theme={theme}>
      <CssBaseline />
      <BrowserRouter>
        <PageContainer>
          <Switch>
            <Route path="/">
              <MainPage />
            </Route>
          </Switch>
        </PageContainer>
      </BrowserRouter>
    </ThemeProvider>
  );
};

export default App;
