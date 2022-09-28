import React from "react";
import { Box, CardContent, Skeleton } from "@mui/material";
import TagChips from "components/TagChips";
import { graphql, useFragment } from "react-relay";
import { ItemDetails_taggedItemNode$key } from "./__generated__/ItemDetails_taggedItemNode.graphql";
import ItemDetailsCardHeader, {
  ItemDetailsCardHeaderSkeleton,
} from "./ItemDetailsCardHeader";
import TrackDetails from "./TrackDetails";
import withQuery from "util/withQuery";
import { ItemDetailsQuery } from "./__generated__/ItemDetailsQuery.graphql";

interface Props {
  taggedItemNodeKey: ItemDetails_taggedItemNode$key;
}

const ItemDetails: React.FC<Props> = ({ taggedItemNodeKey }) => {
  const taggedItemNode = useFragment(
    graphql`
      fragment ItemDetails_taggedItemNode on TaggedItemNode {
        item {
          __typename
          ...ItemDetailsCardHeader_item
          ... on Track {
            ...TrackDetails_track
          }
        }
        ...TagChips_taggedItemNode
      }
    `,
    taggedItemNodeKey
  );

  return (
    <>
      <ItemDetailsCardHeader itemKey={taggedItemNode.item} />
      <CardContent>
        <TagChips taggedItemNodeKey={taggedItemNode} showAdd showDelete />
        {taggedItemNode.item.__typename === "Track" && (
          <Box marginTop={1}>
            <TrackDetails trackKey={taggedItemNode.item} />
          </Box>
        )}
      </CardContent>
    </>
  );
};

export default withQuery<ItemDetailsQuery, Props, "taggedItemNodeKey">({
  query: graphql`
    query ItemDetailsQuery($uri: SpotifyUri!) {
      item(uri: $uri) {
        ...ItemDetails_taggedItemNode
      }
    }
  `,
  dataToProps: (data) => data.item && { taggedItemNodeKey: data.item },
  fallbackElement: (
    <>
      <ItemDetailsCardHeaderSkeleton />
      <CardContent>
        <Skeleton variant="rectangular" height={32} />
      </CardContent>
    </>
  ),
})(ItemDetails);
