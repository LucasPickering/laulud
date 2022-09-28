import React, { useEffect } from "react";
import { useQueryLoader } from "react-relay";
import TagList from "./TagList";
import type { TagListQuery as TagListQueryType } from "./__generated__/TagListQuery.graphql";
import TagListQuery from "./__generated__/TagListQuery.graphql";

interface Props {
  selectedTag?: string;
}

/**
 * Load a list of tags and render the results. If selected tag is passed, that
 * tag will be highlighted in the list (but the parent is responsible for
 * rendering details for the tag).
 */
const TagListLoader: React.FC<Props> = ({ selectedTag }) => {
  const [queryRef, loadQuery] = useQueryLoader<TagListQueryType>(TagListQuery);

  // Load data
  useEffect(() => {
    loadQuery({});
  }, [loadQuery]);

  return <TagList queryRef={queryRef} selectedTag={selectedTag} />;
};

export default TagListLoader;
