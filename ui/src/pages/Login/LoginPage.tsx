import React from "react";
import UnstyledLink from "components/generic/UnstyledLink";
import useRouteQuery from "hooks/useRouteQuery";
import { Button } from "@mui/material";
import { makeStyles } from "@mui/styles";

const useStyles = makeStyles(() => ({
  container: {
    display: "flex",
    flexDirection: "column",
    alignItems: "center",
  },
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
  const classes = useStyles();
  const { next } = useRouteQuery();

  return (
    <div className={classes.container}>
      <Button
        className={classes.spotifyLoginButton}
        component={UnstyledLink}
        to={`/api/oauth/redirect?next=${next ?? "/"}`}
      >
        Log In With Spotify
      </Button>
    </div>
  );
};

export default LoginPage;
