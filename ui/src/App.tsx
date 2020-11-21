import React from "react";
import { CssBaseline } from "@material-ui/core";
import { ThemeProvider } from "@material-ui/styles";
import { BrowserRouter } from "react-router-dom";
import { ReactQueryCacheProvider } from "react-query";

import theme from "./theme";
import queryCache from "./api/queryCache";
import CoreContent from "./components/core/CoreContent";

const App: React.FC = () => {
  return (
    <ReactQueryCacheProvider queryCache={queryCache}>
      <ThemeProvider theme={theme}>
        <CssBaseline />
        <BrowserRouter>
          <CoreContent />
        </BrowserRouter>
      </ThemeProvider>
    </ReactQueryCacheProvider>
  );
};

export default App;
