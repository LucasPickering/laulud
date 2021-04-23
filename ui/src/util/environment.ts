import {
  Environment,
  Network,
  RecordSource,
  Store,
  RequestParameters,
  Variables,
  FetchFunction,
} from "relay-runtime";

const fetchQuery: FetchFunction = (
  operation: RequestParameters,
  variables: Variables
) => {
  return fetch("/api/graphql", {
    method: "POST",
    headers: {
      "Content-Type": "application/json",
    },
    body: JSON.stringify({
      query: operation.text,
      variables,
    }),
  }).then((response) => {
    return response.json();
  });
};

const environment = new Environment({
  network: Network.create(fetchQuery),
  store: new Store(new RecordSource()),
});

export default environment;
