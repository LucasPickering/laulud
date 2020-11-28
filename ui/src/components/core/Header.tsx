import { AppBar, Toolbar, makeStyles } from "@material-ui/core";
import { isEmpty } from "lodash-es";
import React, { useContext } from "react";
import { UserContext } from "util/UserContext";
import HeaderLink from "./HeaderLink";
import LogOutButton from "./LogOutButton";

const LINKS = [
  {
    to: "/tags",
    label: "My Tags",
    exact: false,
  },
  {
    to: "/search",
    label: "Search",
    exact: false,
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
  const showLogOut = !isEmpty(currentUser);

  return (
    <AppBar position="static" color="default">
      <Toolbar component="nav" variant="dense">
        {LINKS.map(({ to, label, exact }) => (
          <HeaderLink key={to} to={to} exact={exact}>
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
