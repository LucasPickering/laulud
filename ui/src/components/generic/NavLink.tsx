import React from "react";
import { NavLink as RouterNavLink } from "react-router-dom";
import { Link as MuiLink } from "@mui/material";

type Props = Omit<React.ComponentProps<typeof RouterNavLinkWrapper>, "style"> &
  React.ComponentProps<typeof MuiLink>;

const NavLink = ({ ...rest }: Props): React.ReactElement => (
  <MuiLink component={RouterNavLinkWrapper} {...rest} />
);

type RouterNavLinkProps = React.ComponentProps<typeof RouterNavLink>;

/**
 * We need another component to wrap React Router's NavLink, to avoid a prop
 * name collision. MUI's Link and NavLink both take a className prop, so in
 * order to forward that prop to NavLink, we have to imitate the old activeClassName
 * behavior from v5.
 */
const RouterNavLinkWrapper = React.forwardRef<
  HTMLAnchorElement,
  {
    to: RouterNavLinkProps["to"];
    end: RouterNavLinkProps["end"];
    className?: string;
    activeClassName?: string;
    style: RouterNavLinkProps["style"];
  }
>(({ className, activeClassName, ...rest }, ref) => (
  <RouterNavLink
    ref={ref}
    className={({ isActive }) =>
      `${className} ${isActive ? activeClassName : ""}`
    }
    {...rest}
  />
));

RouterNavLinkWrapper.displayName = "RouterNavLinkWrapper";

export default NavLink;
