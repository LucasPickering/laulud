import React from "react";
import {
  Box,
  BoxProps,
  CircularProgress,
  CircularProgressProps,
} from "@mui/material";

type Props = BoxProps & Pick<CircularProgressProps, "size">;

/**
 * A loading copmonent that's designed to align well in any space
 */
const Loading: React.FC<Props> = ({ size, ...rest }) => (
  <Box
    display="flex"
    alignItems="center"
    justifyContent="center"
    width="100%"
    height="100%"
    {...rest}
  >
    <CircularProgress size={size} />
  </Box>
);

export default Loading;
