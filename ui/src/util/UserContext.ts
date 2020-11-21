import { CurrentUser } from "api/types";
import React from "react";

export const UserContext = React.createContext<CurrentUser>(
  {} as CurrentUser // this default value never gets used so this is "safe"
);
