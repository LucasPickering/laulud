import React from "react";
import { CardContent } from "@material-ui/core";
import TagChips from "components/TagChips";
import { graphql, useFragment } from "react-relay";
import { ItemDetails_taggedItemNode$key } from "./__generated__/ItemDetails_taggedItemNode.graphql";
import ItemDetailsCardHeader from "./ItemDetailsCardHeader";

const ItemDetails: React.FC<{
  taggedItemNodeKey: ItemDetails_taggedItemNode$key;
}> = ({ taggedItemNodeKey }) => {
  const taggedItemNode = useFragment(
    graphql`
      fragment ItemDetails_taggedItemNode on TaggedItemNode {
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
      </CardContent>
    </>
  );
};

export default ItemDetails;
