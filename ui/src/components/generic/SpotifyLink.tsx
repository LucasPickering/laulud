import React from "react";
import { OpenInNew as IconOpenInNew } from "@material-ui/icons";
import { IconButton, Tooltip } from "@material-ui/core";
import UnstyledLink from "./UnstyledLink";

interface Props {
  readonly item: {
    readonly id: string;
    readonly itemType: string;
  };
}

/**
 * An icon button that opens an item externally in Spotify
 */
const SpotifyLink: React.FC<Props> = ({ item }) => (
  <IconButton
    component={UnstyledLink}
    to={`https://open.spotify.com/${item.itemType}/${item.id}`}
  >
    <Tooltip title="Open in Spotify">
      <IconOpenInNew />
    </Tooltip>
  </IconButton>
);

export default SpotifyLink;
