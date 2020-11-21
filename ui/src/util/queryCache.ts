import { merge } from "lodash-es";
import { QueryCache } from "react-query";

/**
 * A wrapper around the std fetch function, that parses response data as JSON
 * and returns it as a pre-determined type. Note that the response data type
 * IS NOT ACTUALLY VALIDATED - so you better be sure about what the API is
 * returning.
 *
 * @param input same as fetch's input param
 * @param init same as fetch's init param
 * @returns Same as fetch, except the data has been parsed as JSON and type-coerced
 */
export async function queryFn<T>(
  input: RequestInfo,
  init?: RequestInit
): Promise<T> {
  const response = await fetch(
    input,
    merge(
      {
        headers: {
          Accept: "application/json",
        },
      },
      init
    )
  );

  const json = response.json();
  if (response.ok) {
    return json;
  }
  throw json;
}

const queryCache = new QueryCache({
  defaultConfig: {
    queries: {
      queryFn,
      // TODO remove these once the app is more stable
      refetchOnWindowFocus: false,
      retry: 0,
    },
  },
});

export default queryCache;
