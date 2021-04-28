import React from "react";
import { OpenInNew as IconOpenInNew } from "@material-ui/icons";
import { IconButton, Tooltip } from "@material-ui/core";
import UnstyledLink from "./UnstyledLink";
import { graphql, useFragment } from "react-relay";
import { SpotifyLink_taggedItemNode$key } from "./__generated__/SpotifyLink_taggedItemNode.graphql";

interface Props {
  taggedItemNodeKey: SpotifyLink_taggedItemNode$key;
}

/**
 * An icon button that opens an item externally in Spotify
 */
const SpotifyLink: React.FC<Props> = ({ taggedItemNodeKey }) => {
  const taggedItemNode = useFragment(
    graphql`
      # TODO convert to fragment on Item after https://github.com/graphql-rust/juniper/issues/922
      fragment SpotifyLink_taggedItemNode on TaggedItemNode {
        item {
          externalUrls {
            spotify
          }
        }
      }
    `,
    taggedItemNodeKey
  );

  return (
    <IconButton
      component={UnstyledLink}
      to={taggedItemNode.item.externalUrls.spotify}
    >
      <Tooltip title="Open in Spotify">
        <IconOpenInNew />
      </Tooltip>
    </IconButton>
  );
};

export default SpotifyLink;
