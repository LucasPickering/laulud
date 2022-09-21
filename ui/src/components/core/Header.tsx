import { AppBar, Toolbar } from "@mui/material";
import { makeStyles } from "@mui/styles";
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

const useStyles = makeStyles(({ spacing }) => ({
  drawer: {
    width: 150,
  },
  drawerButton: {
    marginRight: spacing(1),
  },
  grow: {
    flexGrow: 1,
  },
}));

/**
 * Site-wide header bar
 */
const Header: React.FC = () => {
  const classes = useStyles();
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
        <div className={classes.grow} />
        {showLogOut && <LogOutButton />}
      </Toolbar>
    </AppBar>
  );
};

export default Header;
