import React from "react";
import { makeStyles } from "@material-ui/core";
import { graphql, useFragment } from "react-relay";
import { UnknownItemTypeError } from "util/errors";
import { ItemArt_taggedItemNode$key } from "./__generated__/ItemArt_taggedItemNode.graphql";

const useStyles = makeStyles(() => ({
  small: {
    width: 48,
    height: 48,
  },
  medium: {
    width: 96,
    height: 96,
  },
  large: {
    // TODO
  },
}));

interface Props {
  taggedItemNodeKey: ItemArt_taggedItemNode$key;
  size?: "small" | "medium" | "large";
}

/**
 * Render an image for a Spotify item. For tracks and albums this will be the
 * album art. For artists it's the artist photo.
 */
function ItemArt({
  taggedItemNodeKey,
  size = "medium",
}: Props): React.ReactElement {
  const classes = useStyles();
  const taggedItemNode = useFragment(
    graphql`
      # TODO convert to fragment on Item after https://github.com/graphql-rust/juniper/issues/922
      fragment ItemArt_taggedItemNode on TaggedItemNode {
        item {
          __typename
          ... on Track {
            album {
              images {
                url
              }
            }
            name
          }
          ... on AlbumSimplified {
            images {
              url
            }
            name
          }
          ... on Artist {
            images {
              url
            }
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
        <img
          className={classes[size]}
          alt={`${item.name} icon`}
          src={item.album.images[0]?.url}
        />
      );

    case "AlbumSimplified":
    case "Artist":
      return (
        <img
          className={classes[size]}
          alt={`${item.name} icon`}
          src={item.images[0]?.url}
        />
      );

    default:
      throw new UnknownItemTypeError(item.__typename);
  }
}

export default ItemArt;
