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
          ...ItemArt_item
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
      }
    `,
    taggedItemNodeKey
  );

  const item = taggedItemNode.item;
  switch (item.__typename) {
    case "Track":
      return (
        <>
          <ListItemAvatar>
            <ItemArt itemKey={item} size="small" />
          </ListItemAvatar>
          <ListItemText
            primary={item.name}
            secondary={item.artists?.map((artist) => artist.name).join(", ")}
          />
        </>
      );

    case "AlbumSimplified":
      return (
        <>
          <ListItemAvatar>
            <ItemArt itemKey={item} size="small" />
          </ListItemAvatar>
          <ListItemText
            primary={item.name}
            secondary={item.artists?.map((artist) => artist.name).join(", ")}
          />
        </>
      );

    case "Artist":
      return (
        <>
          <ListItemAvatar>
            <ItemArt itemKey={item} size="small" />
          </ListItemAvatar>
          <ListItemText primary={item.name} />
        </>
      );

    default:
      throw new UnknownItemTypeError(item.__typename);
  }
}

export default ItemListEntry;
