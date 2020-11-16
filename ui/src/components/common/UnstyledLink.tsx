import React from "react";
import { Link as RouterLink } from "react-router-dom";
import { History, Location, LocationDescriptorObject } from "history";

type Props = React.ComponentProps<typeof RouterLink>;

// This hash code is adapter from https://github.com/rafrex/react-router-hash-link

/**
 * Gets the hash portion of a link target.
 * @param to The link target
 * @return The hash target, or empty string if there is none
 */
function getHashFragment(
  to:
    | string
    | LocationDescriptorObject<unknown>
    | ((location: Location<unknown>) => History.LocationDescriptor<unknown>)
): string {
  let s: string;
  if (typeof to === "string") {
    s = to;
  } else if (typeof to === "object" && typeof to.hash === "string") {
    s = to.hash;
  } else {
    s = "";
  }
  // Get everything after the first #
  return s.substring(s.indexOf("#") + 1);
}

/**
 * An unstyled link, which is used as part of our local Link component, or
 * can be passed to a `component=` prop on other Material UI components. This
 * provides automatic react-router and external link functionality, and also
 * fixes hash links for react-router. You shouldn't even render this directly,
 * only pass it as a `component=` prop. Otherwise, use the local `Link`
 * component.
 *
 * @param to The link target, either local or external
 */
const UnstyledLink = React.forwardRef(
  (
    { to, onClick, children, ...rest }: Props,
    ref: React.Ref<HTMLAnchorElement>
  ): React.ReactElement => {
    const destString = to.toString();
    const external = Boolean(destString.match(/^https?:/));
    const apiLink = Boolean(destString.match(/^\/api\//));

    if (external || apiLink) {
      return (
        <a
          ref={ref}
          href={destString}
          {...(external
            ? {
                target: "_blank",
                rel: "noopener noreferrer",
              }
            : {})}
          {...rest}
        >
          {children}
        </a>
      );
    }

    // Use a react-router link. This needs some special behavior to handle hash
    // links, since those aren't supported natively.
    const hashFragment = getHashFragment(to);

    return (
      <RouterLink
        onClick={(e) => {
          if (onClick) {
            onClick(e);
          }

          if (hashFragment) {
            // Push onto callback queue so it runs after the DOM is updated
            window.setTimeout(() => {
              const element = document.getElementById(hashFragment);
              if (element) {
                element.scrollIntoView();
              }
            });
          }
        }}
        to={to}
        {...rest}
        ref={ref}
      >
        {children}
      </RouterLink>
    );
  }
);

export default UnstyledLink;
