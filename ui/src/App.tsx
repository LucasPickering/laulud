import React from "react";
import { CssBaseline } from "@mui/material";
import { ThemeProvider } from "@mui/material/styles";
import { BrowserRouter } from "react-router-dom";
import { RelayEnvironmentProvider } from "react-relay";
import environment from "util/environment";
import theme from "./theme";
import CoreContent from "./components/core/CoreContent";

const App: React.FC = () => {
  return (
    <RelayEnvironmentProvider environment={environment}>
      <ThemeProvider theme={theme}>
        <CssBaseline />
        <BrowserRouter>
          <CoreContent />
        </BrowserRouter>
      </ThemeProvider>
    </RelayEnvironmentProvider>
  );
};

export default App;
