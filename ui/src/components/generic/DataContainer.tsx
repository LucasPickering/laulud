import React from "react";

import { CircularProgress } from "@material-ui/core";
import { QueryResult } from "react-query";

interface Props<T> extends QueryResult<T> {
  loadingEl?: React.ReactElement;
  children: (data: T) => React.ReactElement;
}

function DataContainer<T>({
  isLoading,
  data,
  error,
  loadingEl = <CircularProgress />,
  children,
}: Props<T>): React.ReactElement | null {
  if (isLoading) {
    return loadingEl;
  }

  if (error) {
    return <div>Error</div>;
  }

  if (data) {
    return children(data);
  }

  // A request hasn't even been attempted yet, just render nothing
  return null;
}

export default DataContainer;
