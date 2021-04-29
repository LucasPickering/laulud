import { CircularProgress } from "@material-ui/core";
import React, { Suspense } from "react";

/**
 * Wrap a component with a <Suspense> tag, so it gets a local loading spinner
 * when loading GraphQL data
 */
function withSuspense<P>(Component: React.FC<P>): React.FC<P> {
  const WrappedComponent: React.FC<P> = (props: P) => (
    <Suspense fallback={<CircularProgress />}>
      <Component {...props} />
    </Suspense>
  );
  WrappedComponent.displayName = `${Component.displayName}Suspense`;

  return WrappedComponent;
}

export default withSuspense;
