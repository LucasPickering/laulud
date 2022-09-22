import { ListItemAvatar, ListItemText } from "@mui/material";
import React from "react";
import { graphql, useFragment } from "react-relay";
import { UnknownItemTypeError } from "util/errors";
import ItemArt from "./generic/ItemArt";
import { ItemListEntry_taggedItemNode$key } from "./__generated__/ItemListEntry_taggedItemNode.graphql";

function ItemListEntry({
  taggedItemNodeKey,
}: {
  taggedItemNodeKey: ItemListEntry_taggedItemNode$key;
}): React.ReactElement {
  const taggedItemNode = useFragment(
    graphql`
      fragment ItemListEntry_taggedItemNode on TaggedItemNode {
        item {
          __typename
          ... on Track {
            name
            artists {
              name
            }
          }
          ... on AlbumSimplified {
            name
            artists {
              name
            }
          }
          ... on Artist {
            name
          }
        }
        ...ItemArt_taggedItemNode
      }
    `,
    taggedItemNodeKey
  );

  switch (taggedItemNode.item.__typename) {
    case "Track":
      return (
        <>
          <ListItemAvatar>
            <ItemArt taggedItemNodeKey={taggedItemNode} size="small" />
          </ListItemAvatar>
          <ListItemText
            primary={taggedItemNode.item.name}
            secondary={taggedItemNode.item.artists
              ?.map((artist) => artist.name)
              .join(", ")}
          />
        </>
      );

    case "AlbumSimplified":
      return (
        <>
          <ListItemAvatar>
            <ItemArt taggedItemNodeKey={taggedItemNode} size="small" />
          </ListItemAvatar>
          <ListItemText
            primary={taggedItemNode.item.name}
            secondary={taggedItemNode.item.artists
              ?.map((artist) => artist.name)
              .join(", ")}
          />
        </>
      );

    case "Artist":
      return (
        <>
          <ListItemAvatar>
            <ItemArt taggedItemNodeKey={taggedItemNode} size="small" />
          </ListItemAvatar>
          <ListItemText primary={taggedItemNode.item.name} />
        </>
      );

    default:
      throw new UnknownItemTypeError(taggedItemNode.item.__typename);
  }
}

export default ItemListEntry;
