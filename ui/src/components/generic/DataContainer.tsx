import React from "react";

import { CircularProgress, makeStyles } from "@material-ui/core";
import { QueryResult } from "react-query";

const useStyles = makeStyles(({ spacing }) => ({
  loadingWrapper: {
    width: "100%",
    height: "100%",
    display: "flex",
    alignItems: "center",
    justifyContent: "center",
    padding: spacing(2),
  },
}));

interface Props<T> extends Pick<QueryResult<T>, "status" | "data"> {
  idleEl?: React.ReactElement | null;
  loadingEl?: React.ReactElement | null;
  errorEl?: React.ReactElement | null;
  children?: (data: T) => React.ReactElement | null;
}

function DefaultLoading(): React.ReactElement {
  const classes = useStyles();

  return (
    <div className={classes.loadingWrapper}>
      <CircularProgress />
    </div>
  );
}

function DataContainer<T>({
  status,
  data,
  idleEl = null,
  loadingEl = <DefaultLoading />,
  errorEl = <div>Error</div>,
  children = () => null,
}: Props<T>): React.ReactElement | null {
  switch (status) {
    case "idle":
      return idleEl;
    case "loading":
      return loadingEl;
    case "error":
      return errorEl;
    case "success":
      // We know data is populated here so we can coerce the type
      return children(data as T);
    default:
      throw new Error(`Invalid status: ${status}`);
  }
}

export default DataContainer;
