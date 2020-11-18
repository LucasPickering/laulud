import React from "react";
import { Link as MuiLink } from "@material-ui/core";

import UnstyledLink from "./UnstyledLink";

type Props = Pick<React.ComponentProps<typeof UnstyledLink>, "to"> &
  Omit<React.ComponentProps<typeof MuiLink>, "component">;

/**
 * A component that merges the styles of Material UI's Link with the functionality
 * of React router's Link. If the given target has a protocol or starts with
 * "/api/", this will assume it's an external link, and use a normal <a> instead
 * of a router link. In the former case, it will also open it in a new tab.
 *
 * DO NOT PASS THIS COMPONENT to a `component=` prop! Use UnstyledLink instead!
 *
 * @param to The link target, either local or external
 */
const Link = (props: Props): React.ReactElement => {
  // This is a dumb hack that somehow gets around a shitty TS error. I don't
  // understand it, but it works ¯\_(ツ)_/¯
  const props2 = {
    component: UnstyledLink,
    ...props,
  };
  return <MuiLink {...props2} />;
};

export default Link;
