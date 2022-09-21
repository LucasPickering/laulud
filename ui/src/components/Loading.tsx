import React from "react";
import { CircularProgress } from "@mui/material";
import { makeStyles } from "@mui/styles";

const useStyles = makeStyles({
  loadingWrapper: {
    display: "flex",
    alignItems: "center",
    justifyContent: "center",
    width: "100%",
    height: "100%",
  },
});

/**
 * A loading copmonent that's designed to align well in any space
 */
const Loading: React.FC<React.ComponentProps<typeof CircularProgress>> = (
  props
) => {
  const classes = useStyles();

  return (
    <div className={classes.loadingWrapper}>
      <CircularProgress {...props} />
    </div>
  );
};

export default Loading;
