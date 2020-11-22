import { QueryCache } from "react-query";
import axios, { AxiosRequestConfig } from "axios";

axios.defaults.headers.common["Accept"] = "application/json";

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
