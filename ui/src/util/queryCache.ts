import { QueryCache } from "react-query";
import axios, { AxiosRequestConfig } from "axios";
import { TaggedItem } from "schema";
import { ApiRoute, ApiRouteItem, ApiRouteItemSearch } from "api";

axios.defaults.headers.common["Accept"] = "application/json";

/**
 * All queries are cached by their API route. This is just a convenience type
 * to make query-related code a bit more readable.
 */
export type LauludQueryKey = ApiRoute;

/**
 * A wrapper around axios, that parses response data as JSON and returns it as
 * a pre-determined type. Note that the response data type IS NOT ACTUALLY
 * VALIDATED - so you better be sure about what the API is returning.
 *
 * @param url Request URL
 * @param config Axios request config
 * @returns Same as axios, except the data has been parsed as JSON and type-coerced
 */
export async function queryFn<T = unknown>(
  config: AxiosRequestConfig
): Promise<T> {
  return axios(config).then((response) => response.data);
}

/**
 * The react-query cache object that stores all query data
 */
export const queryCache = new QueryCache({
  defaultConfig: {
    queries: {
      queryFn,
      refetchOnWindowFocus: false,
    },
  },
});

/**
 * Update all instances of an item (track/album/artist) in the query cache. This
 * should be called any time an item is mutated (generally meaning tags added or
 * deleted). The item will always be updated in the `items` section of the cache
 * (where individually fetched items are stored), and if requested, will be
 * updated in a search result.
 *
 * @param item The new item value, to stored in the cache
 * @param searchQueryKey If specified, the results of this particular search
 *  query will be updated with the new item value. Use this if a list of
 *  search results is visible on the same page where a user can make mutations
 *  to an item, so that the data updates in both places.
 */
export function updateCachedItem(
  item: TaggedItem,
  searchQueryKey?: ApiRouteItemSearch
): void {
  const uri = item.item.data.uri;

  // Update the standalone item in the cache
  const singleItemQueryKey: ApiRouteItem = ["items", uri];
  queryCache.setQueryData<TaggedItem>(singleItemQueryKey, item);

  queryCache.invalidateQueries(searchQueryKey);
  queryCache.invalidateQueries(["tags"]);
}
