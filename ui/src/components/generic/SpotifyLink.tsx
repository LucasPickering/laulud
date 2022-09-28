import React from "react";
import { OpenInNew as IconOpenInNew } from "@mui/icons-material";
import { IconButton, Tooltip } from "@mui/material";
import UnstyledLink from "./UnstyledLink";
import { graphql, useFragment } from "react-relay";
import { SpotifyLink_item$key } from "./__generated__/SpotifyLink_item.graphql";

interface Props {
  itemKey: SpotifyLink_item$key;
}

/**
 * An icon button that opens an item externally in Spotify
 */
const SpotifyLink: React.FC<Props> = ({ itemKey }) => {
  const item = useFragment(
    graphql`
      fragment SpotifyLink_item on Item {
        externalUrls {
          spotify
        }
      }
    `,
    itemKey
  );

  return (
    <IconButton component={UnstyledLink} to={item.externalUrls.spotify}>
      <Tooltip title="Open in Spotify">
        <IconOpenInNew />
      </Tooltip>
    </IconButton>
  );
};

export default SpotifyLink;
