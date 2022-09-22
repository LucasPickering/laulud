import React from "react";
import UnstyledLink from "components/generic/UnstyledLink";
import useRouteQuery from "hooks/useRouteQuery";
import { Box, Button } from "@mui/material";

const LoginPage: React.FC = () => {
  const { next } = useRouteQuery();

  return (
    <Box display="flex" flexDirection="column" alignItems="center">
      <Button
        component={UnstyledLink}
        to={`/api/oauth/redirect?next=${next ?? "/"}`}
        sx={{
          // Very specific rules for Spotify branding
          // https://developer.spotify.com/branding-guidelines/
          height: 48,
          backgroundColor: "#1DB954",
          color: "#FFFFFF",
          borderRadius: 500,
          padding: `18px 48px 16px`,
        }}
      >
        Log In With Spotify
      </Button>
    </Box>
  );
};

export default LoginPage;
