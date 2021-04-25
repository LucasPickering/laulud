import React, { Suspense } from "react";
import { CircularProgress, CssBaseline } from "@material-ui/core";
import { ThemeProvider } from "@material-ui/styles";
import { BrowserRouter } from "react-router-dom";
import { RelayEnvironmentProvider } from "react-relay";
import environment from "util/environment";
import theme from "./theme";
import CoreContent from "./components/core/CoreContent";

const App: React.FC = () => {
  return (
    <RelayEnvironmentProvider environment={environment}>
      <ThemeProvider theme={theme()}>
        <CssBaseline />
        <BrowserRouter>
          <Suspense fallback={<CircularProgress />}>
            <CoreContent />
          </Suspense>
        </BrowserRouter>
      </ThemeProvider>
    </RelayEnvironmentProvider>
  );
};

export default App;
