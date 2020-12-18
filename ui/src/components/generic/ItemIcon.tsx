import React from "react";
import { Tooltip } from "@material-ui/core";
import { Item } from "schema";
import {
  AlbumOutlined as AlbumIcon,
  AudiotrackOutlined as AudiotrackIcon,
  PersonOutlined as PersonIcon,
} from "@material-ui/icons";

function ItemIcon({ item }: { item: Item }): React.ReactElement {
  switch (item.type) {
    case "track":
      return (
        <Tooltip title="Track">
          <AudiotrackIcon />
        </Tooltip>
      );
    case "album":
      return (
        <Tooltip title="Album">
          <AlbumIcon />
        </Tooltip>
      );
    case "artist":
      return (
        <Tooltip title="Artist">
          <PersonIcon />
        </Tooltip>
      );
  }
}

export default ItemIcon;
