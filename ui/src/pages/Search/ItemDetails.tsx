import React from "react";
import { QueryStatus } from "react-query";
import { Card, CardContent, CardHeader, Snackbar } from "@material-ui/core";
import ItemArt from "components/generic/ItemArt";
import NewTagChip from "../../components/NewTagChip";
import TagChips from "components/TagChips";
import SpotifyLink from "components/generic/SpotifyLink";
import { graphql, useFragment, useMutation } from "react-relay";
import { Alert } from "@material-ui/lab";
import {
  ItemDetails_taggedItemNode,
  ItemDetails_taggedItemNode$key,
} from "./__generated__/ItemDetails_taggedItemNode.graphql";
import { UnknownItemTypeError } from "util/errors";

const ItemDetails: React.FC<{
  taggedItemNodeKey: ItemDetails_taggedItemNode$key;
}> = ({ taggedItemNodeKey }) => {
  const taggedItemNode = useFragment(
    graphql`
      fragment ItemDetails_taggedItemNode on TaggedItemNode {
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
        ...TagChips_taggedItemNode
        ...ItemArt_taggedItemNode
        ...SpotifyLink_taggedItemNode
      }
    `,
    taggedItemNodeKey
  );

  return (
    <>
      <Card>
        <ItemHeader taggedItemNode={taggedItemNode} />
        <CardContent>
          <TagChips taggedItemNodeKey={taggedItemNode} showAdd showDelete />
        </CardContent>
      </Card>

      {/* TODO enable snack bar */}
      {/* <Snackbar
        open={createTagStatus === QueryStatus.Error}
        autoHideDuration={5000}
        onClose={() => resetCreateTagStatus()}
      >
        <Alert severity="error">Error creating tag</Alert>
      </Snackbar> */}
    </>
  );
};

const ItemHeader: React.FC<{ taggedItemNode: ItemDetails_taggedItemNode }> = ({
  taggedItemNode,
}) => {
  switch (taggedItemNode.item.__typename) {
    case "Track": {
      const track = taggedItemNode.item;
      return (
        <CardHeader
          title={track.name}
          subheader={track.artists!.map((artist) => artist.name).join(", ")}
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
          subheader={album.artists!.map((artist) => artist.name).join(", ")}
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

export default ItemDetails;
