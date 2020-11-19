/**
 * "fetch data" - a wrapper around the std fetch function, that parses response
 * data as JSON and returns it as a pre-determined type. Note that the response
 * data type IS NOT ACTUALLY VALIDATED - so you better be sure about what the
 * API is returning.
 *
 * @param input same as fetch's input param
 * @param init same as fetch's init param
 * @returns Same as fetch, except the data has been parsed as JSON and type-coerced
 */
const fd = <T>(input: RequestInfo, init?: RequestInit): Promise<T> =>
  // todo set Accept: application/json
  fetch(input, init).then((response) => response.json());

export default fd;
