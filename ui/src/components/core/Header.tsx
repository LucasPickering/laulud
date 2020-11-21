import { AppBar, Toolbar, makeStyles } from "@material-ui/core";
import { isEmpty } from "lodash-es";
import React, { useContext } from "react";
import { UserContext } from "util/UserContext";
import LogOutButton from "./LogOutButton";

const useLocalStyles = makeStyles(({ spacing }) => ({
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
  const localClasses = useLocalStyles();
  const currentUser = useContext(UserContext);
  const showLogOut = !isEmpty(currentUser);

  return (
    <AppBar position="static" color="default">
      <Toolbar component="nav" variant="dense">
        <div className={localClasses.grow} />
        {showLogOut && <LogOutButton />}
      </Toolbar>
    </AppBar>
  );
};

export default Header;
