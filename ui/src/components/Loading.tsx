import React from "react";
import { Box, CircularProgress } from "@mui/material";

/**
 * A loading copmonent that's designed to align well in any space
 */
const Loading: React.FC<React.ComponentProps<typeof CircularProgress>> = (
  props
) => (
  <Box
    display="flex"
    alignItems="center"
    justifyContent="center"
    width="100%"
    height="100%"
  >
    <CircularProgress {...props} />
  </Box>
);

export default Loading;
