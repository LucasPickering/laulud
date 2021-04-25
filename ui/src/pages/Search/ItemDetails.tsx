import React from "react";
import { QueryStatus } from "react-query";
import { Card, CardContent, CardHeader, Snackbar } from "@material-ui/core";
import ItemArt from "components/generic/ItemArt";
import NewTagChip from "../../components/NewTagChip";
import TagChips from "components/TagChips";
import SpotifyLink from "components/generic/SpotifyLink";
import { useFragment } from "react-relay";
import { Alert } from "@material-ui/lab";
import {
  ItemDetails_itemNode,
  ItemDetails_itemNode$key,
} from "./__generated__/ItemDetails_itemNode.graphql";

const ItemHeader: React.FC<{ itemNode: ItemDetails_itemNode }> = ({
  itemNode,
}) => {
  switch (itemNode.item.__typename) {
    case "Track": {
      const track = itemNode.item;
      return (
        <CardHeader
          title={track.name}
          subheader={track.artists.map((artist) => artist.name).join(", ")}
          avatar={<ItemArt item={track.album} />}
          action={<SpotifyLink item={itemNode.item} />}
        />
      );
    }

    case "AlbumSimplified": {
      const album = itemNode.item;
      return (
        <CardHeader
          title={album.name}
          subheader={album.artists.map((artist) => artist.name).join(", ")}
          avatar={<ItemArt item={album} />}
          action={<SpotifyLink item={itemNode.item} />}
        />
      );
    }

    case "Artist": {
      const artist = itemNode.item;
      return (
        <CardHeader
          title={artist.name}
          avatar={<ItemArt item={artist} />}
          action={<SpotifyLink item={itemNode.item} />}
        />
      );
    }

    case "%other":
      throw new Error(`Unknown item type: ${itemNode.item.__typename}`);
  }
};

const ItemDetails: React.FC<{
  itemNodeKey: ItemDetails_itemNode$key;
}> = ({ itemNodeKey }) => {
  const itemNode = useFragment(
    graphql`
      fragment ItemDetails_itemNode on TaggedItemNode {
        item {
          __typename
          ... on Track {
            id
            album {
              name
              images {
                url
              }
            }
            artists {
              name
            }
            name
            itemType
          }
          ... on AlbumSimplified {
            id
            artists {
              name
            }
            images {
              url
            }
            name
            itemType
          }
          ... on Artist {
            id
            images {
              url
            }
            name
            itemType
          }
        }
        ...TagChips_itemNode
      }
    `,
    itemNodeKey
  );

  return (
    <>
      <Card>
        <ItemHeader itemNode={itemNode} />
        <CardContent>
          <TagChips
            tags={itemNode}
            deleteTag={(tag) => deleteTag({ uri, tag })}
          >
            <NewTagChip
              color="primary"
              status={createTagStatus}
              createTag={(tag) => createTag({ uri, tag })}
            />
          </TagChips>
        </CardContent>
      </Card>
      <Snackbar
        open={createTagStatus === QueryStatus.Error}
        autoHideDuration={5000}
        onClose={() => resetCreateTagStatus()}
      >
        <Alert severity="error">Error creating tag</Alert>
      </Snackbar>
    </>
  );
};

export default ItemDetails;
