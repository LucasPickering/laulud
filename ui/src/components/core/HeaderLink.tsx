import React from "react";
import NavLink from "../generic/NavLink";
import { Box, useTheme } from "@mui/material";
import { ClassNames } from "@emotion/react";

/**
 * A link in the top header bar.
 */
const HeaderLink: React.FC<React.ComponentProps<typeof NavLink>> = (props) => {
  // TODO grab theme from emotion in css prop
  const { palette } = useTheme();
  const activeStyles = {
    textDecoration: "none",
    borderBottom: `1px solid ${palette.primary.main}`,
  };

  return (
    <ClassNames>
      {({ css }) => (
        <Box component="span" minWidth={80} textAlign="center">
          <NavLink
            activeClassName={css(activeStyles)}
            sx={{ color: palette.text.primary, "&:hover": activeStyles }}
            {...props}
          />
        </Box>
      )}
    </ClassNames>
  );
};

export default HeaderLink;
