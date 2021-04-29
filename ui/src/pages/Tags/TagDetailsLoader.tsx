import React from "react";
import { graphql, useLazyLoadQuery } from "react-relay";
import withSuspense from "util/withSuspense";
import TagDetails from "./TagDetails";
import { TagDetailsLoaderQuery } from "./__generated__/TagDetailsLoaderQuery.graphql";

interface Props {
  tag: string;
}

/**
 * Load detailed data about a particular tag and render it
 */
const TagDetailsLoader: React.FC<Props> = ({ tag }) => {
  // useLazyLoad is discouraged in the relay docs, but it works "well enough"
  // so let's just run with it. For Optimal Peformance™ we could switch to
  // usePreloadedQuery and trigger the query when the new tag is selected
  const data = useLazyLoadQuery<TagDetailsLoaderQuery>(
    graphql`
      query TagDetailsLoaderQuery($tag: String!) {
        tag(tag: $tag) {
          ...TagDetails_tagNode
        }
      }
    `,
    { tag }
  );

  return <TagDetails tagNodeKey={data.tag} />;
};

export default withSuspense(TagDetailsLoader);
