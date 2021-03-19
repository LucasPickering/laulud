import { ApiRouteItemSearch } from "api";
import { MutationResultPair, useMutation } from "react-query";
import { SpotifyUri, TaggedItem } from "schema";
import { queryFn, updateCachedItem } from "util/queryCache";

interface MutationArgs {
  uri: SpotifyUri;
  tag: string;
}

/**
 * A react-query mutation to create a new tag for a Spotify item. Both
 * the item and tagged are specified at mutation time.
 * @param searchQueryKey If specified, the results of this particular search
 *  query will be updated with the new item value. Use this if a list of
 *  search results is visible on the same page where the user is adding the tag.
 * @returns Mutation functions
 */
function useMutationNewItemTag(
  searchQueryKey?: ApiRouteItemSearch
): MutationResultPair<TaggedItem, unknown, MutationArgs, unknown> {
  return useMutation(
    ({ uri, tag }: MutationArgs) =>
      queryFn<TaggedItem>({
        url: `/api/items/${uri}/tags`,
        method: "POST",
        data: { tag },
      }),
    { onSuccess: (data) => updateCachedItem(data, searchQueryKey) }
  );
}

export default useMutationNewItemTag;
