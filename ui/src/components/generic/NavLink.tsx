import React from "react";
import { NavLink as RouterNavLink } from "react-router-dom";
import { Link as MuiLink } from "@material-ui/core";
import { makeStyles } from "@material-ui/core";

const useLocalStyles = makeStyles({
  active: {
    textDecoration: "underline",
  },
});

type Props = Pick<
  React.ComponentProps<typeof RouterNavLink>,
  "to" | "exact" | "activeClassName"
> &
  React.ComponentProps<typeof MuiLink>;

const NavLink = ({ ...rest }: Props): React.ReactElement => {
  const localClasses = useLocalStyles();
  const props = {
    component: RouterNavLink,
    activeClassName: localClasses.active,
    ...rest,
  };

  return <MuiLink {...props} />;
};

export default NavLink;
