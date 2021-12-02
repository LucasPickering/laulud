import React from "react";
import { Navigate } from "react-router-dom";
import { useLocation } from "react-router-dom";

/**
 * Redirect an unauthenticated user to the login page.
 *
 * TODO replace this with a friendly landing page instead.
 */
const NotLoggedInRedirect: React.FC = () => {
  const location = useLocation();

  // Include a param on the login page to tell the server where
  // to send us back to after login
  const currentLocation = `${location.pathname}${location.search}${location.hash}`;
  const search = `?next=${encodeURIComponent(currentLocation)}`;

  return (
    <Navigate
      replace
      to={{
        pathname: "/",
        search,
      }}
    />
  );
};

export default NotLoggedInRedirect;
