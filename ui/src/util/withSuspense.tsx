import React, { Suspense } from "react";
import Loading from "components/Loading";

/**
 * A little helper to fix type errors surrounding React "intrinsic attributes".
 * I don't really understand why this works, but it seems to so that's neat.
 * https://stackoverflow.com/a/59917277/1907353
 */
type Fix<P> = P & {}; // eslint-disable-line @typescript-eslint/ban-types

/**
 * Wrap a component with a <Suspense> tag, so it gets a local loading spinner
 * when loading GraphQL data
 */
function withSuspense<P>(Component: React.FC<Fix<P>>): React.FC<Fix<P>> {
  const WrappedComponent: React.FC<Fix<P>> = (props) => (
    <Suspense fallback={<Loading />}>
      <Component {...props} />
    </Suspense>
  );
  WrappedComponent.displayName = `${Component.displayName}Suspense`;

  return WrappedComponent;
}

export default withSuspense;
