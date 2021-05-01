import { useEffect, useState } from "react";
import {
  // eslint-disable-next-line no-restricted-syntax
  useMutation as useMutationRelay,
  UseMutationConfig,
} from "react-relay";
import {
  Disposable,
  GraphQLTaggedNode,
  IEnvironment,
  MutationConfig,
  MutationParameters,
} from "relay-runtime";

export type MutationStatus = "idle" | "loading" | "success" | "error";

/**
 * A wrapper around Relay's useMutation. Takes the same inputs, but has
 * expanded output to get more granular tracking of the mutation's state.
 * The status will reflect the most recent call to the mutation.
 */
function useMutation<TMutation extends MutationParameters>(
  mutation: GraphQLTaggedNode,
  commitMutationFn?: (
    environment: IEnvironment,
    config: MutationConfig<TMutation>
  ) => Disposable
): {
  commit: (config: UseMutationConfig<TMutation>) => Disposable;
  status: MutationStatus;
  resetStatus: () => void;
} {
  const [commit, isInFlight] = useMutationRelay(mutation, commitMutationFn);
  const [status, setStatus] = useState<MutationStatus>("idle");

  useEffect(() => {
    if (isInFlight) {
      setStatus("loading");
    }
  }, [isInFlight]);

  return {
    commit: ({ onCompleted, onError, ...rest }) =>
      commit({
        // Track success and error status
        onCompleted: (response, errors) => {
          if (errors) {
            setStatus("error");
          } else {
            setStatus("success");
          }
          if (onCompleted) {
            onCompleted(response, errors);
          }
        },
        onError: (error) => {
          setStatus("error");
          if (onError) {
            onError(error);
          }
        },
        ...rest,
      }),
    status,
    resetStatus: () => setStatus("idle"),
  };
}

export default useMutation;
