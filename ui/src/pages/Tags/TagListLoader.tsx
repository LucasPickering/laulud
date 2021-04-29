import React from "react";
import { graphql, useLazyLoadQuery } from "react-relay";
import withSuspense from "util/withSuspense";
import TagList from "./TagList";
import { TagListLoaderQuery } from "./__generated__/TagListLoaderQuery.graphql";

interface Props {
  selectedTag?: string;
}

/**
 * Load a list of tags and render the results. If selected tag is passed, that
 * tag will be highlighted in the list (but the parent is responsible for
 * rendering details for the tag).
 */
const TagListLoader: React.FC<Props> = ({ selectedTag }) => {
  // useLazyLoad is discouraged in the relay docs, but it works "well enough"
  // so let's just run with it. For Optimal Peformance™ we could switch to
  // usePreloadedQuery and trigger the query when the new tag is selected
  const data = useLazyLoadQuery<TagListLoaderQuery>(
    graphql`
      query TagListLoaderQuery {
        tags {
          ...TagList_tagConnection
        }
      }
    `,
    {}
  );

  return <TagList tagConnectionKey={data.tags} selectedTag={selectedTag} />;
};

export default withSuspense(TagListLoader);
