import React from "react";
import { CardContent } from "@mui/material";
import TagChips from "components/TagChips";
import { graphql, useFragment } from "react-relay";
import { ItemDetails_taggedItemNode$key } from "./__generated__/ItemDetails_taggedItemNode.graphql";
import ItemDetailsCardHeader from "./ItemDetailsCardHeader";
import TrackDetails from "./TrackDetails";

const ItemDetails: React.FC<{
  taggedItemNodeKey: ItemDetails_taggedItemNode$key;
}> = ({ taggedItemNodeKey }) => {
  const taggedItemNode = useFragment(
    graphql`
      fragment ItemDetails_taggedItemNode on TaggedItemNode {
        item {
          __typename
          ... on Track {
            ...TrackDetails_track
          }
        }
        ...ItemDetailsCardHeader_taggedItemNode
        ...TagChips_taggedItemNode
      }
    `,
    taggedItemNodeKey
  );

  return (
    <>
      <ItemDetailsCardHeader taggedItemNodeKey={taggedItemNode} />
      <CardContent>
        <TagChips taggedItemNodeKey={taggedItemNode} showAdd showDelete />
        {taggedItemNode.item.__typename === "Track" && (
          <TrackDetails trackKey={taggedItemNode.item} />
        )}
      </CardContent>
    </>
  );
};

export default ItemDetails;
