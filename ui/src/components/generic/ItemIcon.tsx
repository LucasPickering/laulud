import React from "react";
import { Tooltip } from "@mui/material";
import {
  AlbumOutlined as AlbumIcon,
  AudiotrackOutlined as AudiotrackIcon,
  PersonOutlined as PersonIcon,
} from "@mui/icons-material";
import { graphql, useFragment } from "react-relay";
import { ItemIcon_item$key } from "./__generated__/ItemIcon_item.graphql";
import { UnknownItemTypeError } from "util/errors";

interface Props {
  itemKey: ItemIcon_item$key;
}

function ItemIcon({ itemKey }: Props): React.ReactElement {
  const item = useFragment(
    graphql`
      fragment ItemIcon_item on Item {
        __typename
      }
    `,
    itemKey
  );

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
