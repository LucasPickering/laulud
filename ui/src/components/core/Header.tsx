import { AppBar, Box, Toolbar } from "@mui/material";
import React, { useContext } from "react";
import { UserContext } from "util/UserContext";
import HeaderLink from "./HeaderLink";
import LogOutButton from "./LogOutButton";

const LINKS = [
  {
    to: "/tags",
    label: "My Tags",
    end: false,
  },
  {
    to: "/search",
    label: "Search",
    end: false,
  },
];

/**
 * Site-wide header bar
 */
const Header: React.FC = () => {
  const currentUser = useContext(UserContext);
  const showLogOut = currentUser.isLoggedIn;

  return (
    <AppBar position="static" color="default">
      <Toolbar component="nav" variant="dense">
        {LINKS.map(({ to, label, end }) => (
          <HeaderLink key={to} to={to} end={end}>
            {label}
          </HeaderLink>
        ))}
        <Box flexGrow={1} />
        {showLogOut && <LogOutButton />}
      </Toolbar>
    </AppBar>
  );
};

export default Header;
