import React from "react";
import { graphql, useFragment } from "react-relay";
import { CardHeader } from "@mui/material";
import SpotifyLink from "components/generic/SpotifyLink";
import ItemArt from "components/generic/ItemArt";
import { UnknownItemTypeError } from "util/errors";
import { ItemDetailsCardHeader_item$key } from "./__generated__/ItemDetailsCardHeader_item.graphql";

const ItemDetailsCardHeader: React.FC<{
  itemKey: ItemDetailsCardHeader_item$key;
}> = ({ itemKey }) => {
  const item = useFragment(
    graphql`
      fragment ItemDetailsCardHeader_item on Item {
        __typename
        uri
        externalUrls {
          spotify
        }
        ...ItemArt_item
        ...SpotifyLink_item
        ... on Track {
          artists {
            name
          }
          name
        }
        ... on AlbumSimplified {
          artists {
            name
          }
          name
        }
        ... on Artist {
          name
        }
      }
    `,
    itemKey
  );

  switch (item.__typename) {
    case "Track": {
      return (
        <CardHeader
          title={item.name}
          subheader={item.artists?.map((artist) => artist.name).join(", ")}
          avatar={<ItemArt itemKey={item} />}
          action={<SpotifyLink itemKey={item} />}
        />
      );
    }

    case "AlbumSimplified": {
      return (
        <CardHeader
          title={item.name}
          subheader={item.artists?.map((artist) => artist.name).join(", ")}
          avatar={<ItemArt itemKey={item} />}
          action={<SpotifyLink itemKey={item} />}
        />
      );
    }

    case "Artist": {
      return (
        <CardHeader
          title={item.name}
          avatar={<ItemArt itemKey={item} />}
          action={<SpotifyLink itemKey={item} />}
        />
      );
    }

    default:
      throw new UnknownItemTypeError(item.__typename);
  }
};

export default ItemDetailsCardHeader;
