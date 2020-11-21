import React from "react";

import UnstyledLink from "components/generic/UnstyledLink";
import useRouteQuery from "hooks/useRouteQuery";
import { Button, Grid, makeStyles } from "@material-ui/core";

const useLocalStyles = makeStyles(({ palette, spacing }) => ({
  spotifyLoginButton: {
    // Very specific rules cause Spotify branding
    // https://developer.spotify.com/branding-guidelines/
    height: 48,
    backgroundColor: "#1DB954",
    color: "#FFFFFF",
    borderRadius: 500,
    padding: `18px 48px 16px`,
  },
}));

const LoginPage: React.FC = () => {
  const localClasses = useLocalStyles();
  const { next } = useRouteQuery();

  return (
    <Grid container justifyContent="center">
      <Grid item xs={12} sm={6} md={4}>
        <Button
          className={localClasses.spotifyLoginButton}
          component={UnstyledLink}
          to={`/api/oauth/redirect?next=${next ?? "/"}`}
        >
          Log In With Spotify
        </Button>
      </Grid>
    </Grid>
  );
};

export default LoginPage;
