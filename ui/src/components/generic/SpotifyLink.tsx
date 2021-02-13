import React from "react";
import { OpenInNew as IconOpenInNew } from "@material-ui/icons";
import { IconButton, Tooltip } from "@material-ui/core";

import UnstyledLink from "./UnstyledLink";
import { Item } from "schema";

interface Props {
  item: Item;
}

/**
 * An icon button that opens an item externally in Spotify
 */
const SpotifyLink: React.FC<Props> = ({ item }) => (
  <IconButton
    component={UnstyledLink}
    to={`https://open.spotify.com/${item.type}/${item.data.id}`}
  >
    <Tooltip title="Open in Spotify">
      <IconOpenInNew />
    </Tooltip>
  </IconButton>
);

export default SpotifyLink;
