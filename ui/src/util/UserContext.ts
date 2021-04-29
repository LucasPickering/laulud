import React from "react";

export interface UserContextType {
  isLoggedIn: boolean;
}

export const UserContext = React.createContext<UserContextType>({
  isLoggedIn: false,
});
