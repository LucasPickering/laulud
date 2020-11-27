import React from "react";
import { QueryResult } from "react-query";
import { Alert, CircularProgress, makeStyles } from "@material-ui/core";
import Link from "./Link";

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

function DefaultError(): React.ReactElement {
  return (
    <Alert severity="error">
      An error occurred. Try again or maybe{" "}
      <Link to="https://github.com/LucasPickering/laulud/issues/new">
        file an issue
      </Link>
      .
    </Alert>
  );
}

function DataContainer<T>({
  status,
  data,
  idleEl = null,
  loadingEl = <DefaultLoading />,
  errorEl = <DefaultError />,
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
