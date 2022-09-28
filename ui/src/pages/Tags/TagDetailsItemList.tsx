import React from "react";
import ItemList from "components/ItemList";
import { graphql, useFragment } from "react-relay";
import withQuery from "util/withQuery";
import Loading from "components/Loading";
import { TagDetailsItemListQuery } from "./__generated__/TagDetailsItemListQuery.graphql";
import { TagDetailsItemList_tagNode$key } from "./__generated__/TagDetailsItemList_tagNode.graphql";

interface Props {
  tagNodeKey: TagDetailsItemList_tagNode$key;
}

/**
 * The list of items that a tag has been applied to
 */
const TagDetailsItemList: React.FC<Props> = ({ tagNodeKey }) => {
  const tagNode = useFragment(
    graphql`
      fragment TagDetailsItemList_tagNode on TagNode {
        items {
          ...ItemList_taggedItemConnection
        }
      }
    `,
    tagNodeKey
  );

  return <ItemList taggedItemConnectionKey={tagNode.items} showIcons />;
};

export default withQuery<TagDetailsItemListQuery, Props, "tagNodeKey">({
  query: graphql`
    query TagDetailsItemListQuery($tag: Tag!) {
      tag(tag: $tag) {
        ...TagDetailsItemList_tagNode
      }
    }
  `,
  dataToProps: (data) => data.tag && { tagNodeKey: data.tag },
  // Lists are ugly af w/ skeleton, so stick to loading icon for now
  fallbackElement: <Loading />,
})(TagDetailsItemList);
