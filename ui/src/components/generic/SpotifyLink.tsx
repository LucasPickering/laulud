import React from "react";
import { OpenInNew as IconOpenInNew } from "@material-ui/icons";
import { IconButton, Tooltip } from "@material-ui/core";
import UnstyledLink from "./UnstyledLink";

interface Props {
  readonly item: {
    readonly externalUrls: {
      readonly spotify: string;
    };
  };
}

/**
 * An icon button that opens an item externally in Spotify
 */
const SpotifyLink: React.FC<Props> = ({ item }) => (
  <IconButton component={UnstyledLink} to={item.externalUrls.spotify}>
    <Tooltip title="Open in Spotify">
      <IconOpenInNew />
    </Tooltip>
  </IconButton>
);

export default SpotifyLink;
