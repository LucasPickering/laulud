import React from "react";
import { Tooltip } from "@material-ui/core";
import {
  AlbumOutlined as AlbumIcon,
  AudiotrackOutlined as AudiotrackIcon,
  PersonOutlined as PersonIcon,
} from "@material-ui/icons";
import { graphql, useFragment } from "react-relay";
import { ItemIcon_taggedItemNode$key } from "./__generated__/ItemIcon_taggedItemNode.graphql";
import { UnknownItemTypeError } from "util/errors";

interface Props {
  taggedItemNodeKey: ItemIcon_taggedItemNode$key;
}

function ItemIcon({ taggedItemNodeKey }: Props): React.ReactElement {
  const taggedItemNode = useFragment(
    graphql`
      # TODO convert to fragment on Item after https://github.com/graphql-rust/juniper/issues/922
      fragment ItemIcon_taggedItemNode on TaggedItemNode {
        item {
          __typename
        }
      }
    `,
    taggedItemNodeKey
  );
  const item = taggedItemNode.item;

  switch (item.__typename) {
    case "Track":
      return (
        <Tooltip title="Track">
          <AudiotrackIcon />
        </Tooltip>
      );
    case "AlbumSimplified":
      return (
        <Tooltip title="Album">
          <AlbumIcon />
        </Tooltip>
      );
    case "Artist":
      return (
        <Tooltip title="Artist">
          <PersonIcon />
        </Tooltip>
      );
    default:
      throw new UnknownItemTypeError(item.__typename);
  }
}

export default ItemIcon;
