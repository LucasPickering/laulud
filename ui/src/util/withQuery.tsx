import React, { Suspense } from "react";
import { PreloadedQuery, usePreloadedQuery } from "react-relay";
import { GraphQLTaggedNode, OperationType } from "relay-runtime";

interface Options<
  Query extends OperationType,
  Props,
  DataKeys extends keyof Props
> {
  query: GraphQLTaggedNode;
  dataToProps: (
    data: Query["response"]
  ) => Pick<Props, DataKeys> | null | undefined;
  fallbackElement: React.ReactElement | null;
  preloadElement?: React.ReactElement | null;
  noDataElement?: React.ReactElement | null;
}

/**
 * Get a type for the props of a wrapping component, given:
 * - Wrapper props (T)
 * - Underlying props, defined by the wrapped component (Props)
 * - Data props, defined by the wrapped component by derived from Relay state
 */
type AllProps<
  OuterProps,
  InnerProps,
  DataKeys extends keyof InnerProps
> = OuterProps &
  // We want to only pull out the static props from `Props`. Generally, `Props`
  // shouldn't intersect with `T`, but I included that in the `Omit` in an
  // attempt to convince TS that what I'm doing is legit. It didn't work, but
  // I'm leaving it there to make myself feel better.
  Omit<InnerProps, DataKeys | keyof OuterProps>;

type LoaderProps<
  Query extends OperationType,
  InnerProps,
  DataKeys extends keyof InnerProps
> = AllProps<{ queryRef: PreloadedQuery<Query> }, InnerProps, DataKeys>;

type SuspenseProps<
  Query extends OperationType,
  InnerProps,
  DataKeys extends keyof InnerProps
> = AllProps<
  { queryRef: PreloadedQuery<Query> | null | undefined },
  InnerProps,
  DataKeys
>;

/**
 * Higher-order component to wrap a component with the logic necessary to
 * provide it with GraphQL query data. A parent of this component must load
 * the query, but this HoC will supply the usePreloadedQuery call needed to
 * grab that data and start loading fragments. It will also provide a Suspense
 * layer, so that when the query is undefined/loading, the suspense fallback
 * will be shown.
 *
 * This is a two-stage curried function, meaning a call looks like:
 *
 * ```
 * withQuery({ ...options })(Component)
 * ```
 *
 * The returned component will support pass-through props, meaning any prop
 * not returned from the `dataToProps` param should be passed by the parent.
 *
 * @param query GraphQL query definition supplying data
 * @param dataToProps Function to map the query response to the child component's
 *  props. Return null/undefined if data is missing, which will render the
 *  no-data element instead of the child component
 * @param fallbackElement Element to render while query is loading (via Suspense)
 * @param preloadElement Element to render before the first load is requested
 * @param noDataElement Element to render when the API didn't return the
 *  requested data (typically indicating a 404-type error)
 * @returns Wrapped component
 */
function withQuery<
  Query extends OperationType,
  Props,
  DataKeys extends keyof Props = keyof Props
>({
  query,
  dataToProps,
  fallbackElement,
  preloadElement = null,
  noDataElement = null,
}: Options<Query, Props, DataKeys>): (
  Component: React.FC<Props>
) => React.FC<SuspenseProps<Query, Props, DataKeys>> {
  return (Component) => {
    const baseName = Component.displayName ?? Component.name;

    // We need two separate components here: Loader loads the query data when
    // the query has been executed, Suspense shows loading status when it hasn't.
    // This is two components because hooks can't be optional, we can only do
    // optional logic at the component boundary (when queryRef is null)
    const LoaderComponent: React.FC<LoaderProps<Query, Props, DataKeys>> = ({
      queryRef,
      ...rest
    }) => {
      const data = usePreloadedQuery<Query>(query, queryRef);
      const dataProps = dataToProps(data);

      if (!dataProps) {
        return noDataElement;
      }

      // The props that we'll transparently pass through the wrappers
      // Unfortunate type assertion. I'm 99% sure I set up all the types
      // correctly here, TS just isn't complicated enough to do these assertions
      const props = {
        ...rest,
        ...dataProps,
      } as Props & {}; // eslint-disable-line @typescript-eslint/ban-types
      return <Component {...props} />;
    };
    LoaderComponent.displayName = `${baseName}Loader`;

    const SuspenseComponent: React.FC<
      SuspenseProps<Query, Props, DataKeys>
    > = ({ queryRef, ...rest }) => {
      if (!queryRef) {
        return preloadElement;
      }

      return (
        <Suspense fallback={fallbackElement}>
          {/* I can't figure out how to fix this error. This whole thing is fucked.
           eslint-disable-next-line @typescript-eslint/ban-ts-comment
           @ts-ignore 2122 */}
          <LoaderComponent queryRef={queryRef} {...rest} />
        </Suspense>
      );
    };
    SuspenseComponent.displayName = `${baseName}Suspense`;

    return SuspenseComponent;
  };
}

export default withQuery;
