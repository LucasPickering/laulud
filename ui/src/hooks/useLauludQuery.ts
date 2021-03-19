import { ApiOutputType, ApiRoute } from "api";
import { AxiosRequestConfig } from "axios";
import { QueryConfig, QueryResult, useQuery } from "react-query";
import { queryFn } from "util/queryCache";

/**
 * Query the API. Wrapper around react-query's useQuery, with added type
 * enforcement. This hook will infer its own output type based on the given
 * API route (because we know what type each API endpoint returns).
 *
 * **This should be used for all GET requests**
 *
 * @param route The API route to query, in array format. E.g. a GET request to
 *  `/api/items/search/potato` would be `['items', 'search', 'potato']`
 * @param requestConfig Additional fields (beyond the URL) to pass to axios
 *  when making the HTTP request
 * @param queryConfig Additional config fields to pass to react-query, to
 *  control how the request is handled or cached
 * @returns The query state (same as the default useQuery)
 */
function useLauludQuery<T extends ApiRoute, TError = unknown>(
  route: T,
  // Don't allow user to specify URL, because we build it from the route
  requestConfig?: Omit<AxiosRequestConfig, "url">,
  queryConfig?: QueryConfig<ApiOutputType<T>, TError>
): QueryResult<ApiOutputType<T>, TError> {
  const routeEncoded = (route as string[]).map(encodeURIComponent).join("/");
  const url = `/api/${routeEncoded}`;

  // I am the one who queries
  // eslint-disable-next-line no-restricted-syntax
  return useQuery(
    // We assume that a route uniquely identifies a resource, so we can
    // use the route as a key into the query cache. If two locations request
    // the same route, then they're fine sharing data
    route,
    () => queryFn({ ...requestConfig, url }),
    queryConfig
  );
}

export default useLauludQuery;
