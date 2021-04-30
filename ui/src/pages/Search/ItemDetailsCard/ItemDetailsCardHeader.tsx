import React from "react";
import { graphql, useFragment } from "react-relay";
import { CardHeader } from "@material-ui/core";
import SpotifyLink from "components/generic/SpotifyLink";
import ItemArt from "components/generic/ItemArt";
import { UnknownItemTypeError } from "util/errors";
import { ItemDetailsCardHeader_taggedItemNode$key } from "./__generated__/ItemDetailsCardHeader_taggedItemNode.graphql";

const ItemDetailsCardHeader: React.FC<{
  taggedItemNodeKey: ItemDetailsCardHeader_taggedItemNode$key;
}> = ({ taggedItemNodeKey }) => {
  const taggedItemNode = useFragment(
    graphql`
      # TODO convert to fragment on Item after https://github.com/graphql-rust/juniper/issues/922
      fragment ItemDetailsCardHeader_taggedItemNode on TaggedItemNode {
        item {
          __typename
          uri
          externalUrls {
            spotify
          }
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
        ...ItemArt_taggedItemNode
        ...SpotifyLink_taggedItemNode
      }
    `,
    taggedItemNodeKey
  );

  switch (taggedItemNode.item.__typename) {
    case "Track": {
      const track = taggedItemNode.item;
      return (
        <CardHeader
          title={track.name}
          subheader={track.artists?.map((artist) => artist.name).join(", ")}
          avatar={<ItemArt taggedItemNodeKey={taggedItemNode} />}
          action={<SpotifyLink taggedItemNodeKey={taggedItemNode} />}
        />
      );
    }

    case "AlbumSimplified": {
      const album = taggedItemNode.item;
      return (
        <CardHeader
          title={album.name}
          subheader={album.artists?.map((artist) => artist.name).join(", ")}
          avatar={<ItemArt taggedItemNodeKey={taggedItemNode} />}
          action={<SpotifyLink taggedItemNodeKey={taggedItemNode} />}
        />
      );
    }

    case "Artist": {
      const artist = taggedItemNode.item;
      return (
        <CardHeader
          title={artist.name}
          avatar={<ItemArt taggedItemNodeKey={taggedItemNode} />}
          action={<SpotifyLink taggedItemNodeKey={taggedItemNode} />}
        />
      );
    }

    default:
      throw new UnknownItemTypeError(taggedItemNode.item.__typename);
  }
};

export default ItemDetailsCardHeader;
