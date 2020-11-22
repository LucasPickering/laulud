import React from "react";
import { makeStyles } from "@material-ui/core";
import clsx from "clsx";
import NavLink from "../generic/NavLink";

const useLocalStyles = makeStyles(({ palette, transitions, typography }) => {
  const activeStyles = {
    textDecoration: "none",
    borderBottomColor: palette.primary.main,
  };
  return {
    linkContainer: {
      minWidth: 80,
      textAlign: "center",
    },
    link: {
      color: palette.text.primary,
      borderBottom: "1px solid #00000000",
      transitionProperty: "border-bottom, color",
      transitionDuration: `${transitions.duration.short}ms`,
      transitionTimingFunction: "linear",
      ...typography.body1,

      "&:hover, &:active": {
        ...activeStyles,
        color: palette.text.secondary,
      },
    },
    active: activeStyles,
  };
});

/**
 * A link in the top header bar.
 */
const HeaderLink: React.FC<React.ComponentProps<typeof NavLink>> = ({
  className,
  ...rest
}) => {
  const localClasses = useLocalStyles();
  return (
    <span className={localClasses.linkContainer}>
      <NavLink
        className={clsx(localClasses.link, className)}
        activeClassName={localClasses.active}
        {...rest}
      />
    </span>
  );
};

export default HeaderLink;
