import React from "react";
import { graphql, useFragment } from "react-relay";
import { UnknownItemTypeError } from "util/errors";
import { ItemArt_item$key } from "./__generated__/ItemArt_item.graphql";

const sizes = {
  small: {
    width: 48,
    height: 48,
  },
  medium: {
    width: 96,
    height: 96,
  },
};

interface Props {
  itemKey: ItemArt_item$key;
  size?: "small" | "medium";
}

/**
 * Render an image for a Spotify item. For tracks and albums this will be the
 * album art. For artists it's the artist photo.
 */
function ItemArt({ itemKey, size = "medium" }: Props): React.ReactElement {
  const item = useFragment(
    graphql`
      fragment ItemArt_item on Item {
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
    `,
    itemKey
  );

  switch (item.__typename) {
    case "Track":
      return (
        <img
          alt={`${item.name} icon`}
          src={item.album.images[0]?.url}
          css={sizes[size]}
        />
      );

    case "AlbumSimplified":
    case "Artist":
      return (
        <img
          alt={`${item.name} icon`}
          src={item.images[0]?.url}
          css={sizes[size]}
        />
      );

    default:
      throw new UnknownItemTypeError(item.__typename);
  }
}

export default ItemArt;
