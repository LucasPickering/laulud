import React, { Suspense } from "react";
import Loading from "components/Loading";

/**
 * A little helper that strips out "instrinsic attributes" (i.e. HTML attributes)
 * from a props object. In practice component props and intrinsic attributes
 * should never conflict so this is just academic, but it makes TS happy.
 */
type ExcludeIntrinsic<P> = Exclude<P, keyof JSX.IntrinsicAttributes>;

/**
 * Wrap a component with a <Suspense> tag, so it gets a local loading spinner
 * when loading GraphQL data
 */
function withSuspense<P>(
  Component: React.FC<ExcludeIntrinsic<P>>
): React.FC<ExcludeIntrinsic<P>> {
  const WrappedComponent: React.FC<ExcludeIntrinsic<P>> = (props) => (
    <Suspense fallback={<Loading />}>
      <Component {...props} />
    </Suspense>
  );
  WrappedComponent.displayName = `${Component.displayName}Suspense`;

  return WrappedComponent;
}

export default withSuspense;
