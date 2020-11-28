import React from "react";

import { CurrentUser } from "schema";

export const UserContext = React.createContext<CurrentUser>(
  {} as CurrentUser // this default value never gets used so this is "safe"
);
