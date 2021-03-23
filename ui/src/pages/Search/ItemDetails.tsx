import React from "react";
import { QueryStatus } from "react-query";
import { Card, CardContent, CardHeader, Snackbar } from "@material-ui/core";
import { Item, SpotifyUri } from "schema";
import ItemArt from "components/generic/ItemArt";
import DataContainer from "components/generic/DataContainer";
import NewTagChip from "../../components/NewTagChip";
import TagChips from "components/TagChips";
import SpotifyLink from "components/generic/SpotifyLink";
import useMutationNewItemTag from "hooks/useMutationNewItemTag";
import useMutationDeleteItemTag from "hooks/useMutationDeleteItemTag";
import useLauludQuery from "hooks/useLauludQuery";
import { ApiRouteItemSearch } from "api";
import { Alert } from "@material-ui/lab";

const ItemHeader: React.FC<{ item: Item }> = ({ item }) => {
  if (item.type === "track") {
    const track = item.data;
    return (
      <CardHeader
        title={track.name}
        subheader={track.artists.map((artist) => artist.name).join(", ")}
        avatar={<ItemArt item={track.album} />}
        action={<SpotifyLink item={item} />}
      />
    );
  }

  if (item.type === "album") {
    const album = item.data;
    return (
      <CardHeader
        title={album.name}
        subheader={album.artists.map((artist) => artist.name).join(", ")}
        avatar={<ItemArt item={album} />}
        action={<SpotifyLink item={item} />}
      />
    );
  }

  if (item.type === "artist") {
    const artist = item.data;
    return (
      <CardHeader
        title={artist.name}
        avatar={<ItemArt item={artist} />}
        action={<SpotifyLink item={item} />}
      />
    );
  }

  throw new Error(`Invalid item: ${item}`);
};

const ItemDetails: React.FC<{
  uri: SpotifyUri;
  searchQueryKey?: ApiRouteItemSearch;
}> = ({ uri, searchQueryKey }) => {
  const { data: track, ...state } = useLauludQuery(["items", uri]);
  const [
    createTag,
    { status: createTagStatus, reset: resetCreateTagStatus },
  ] = useMutationNewItemTag(searchQueryKey);
  const [deleteTag] = useMutationDeleteItemTag(searchQueryKey);

  return (
    <>
      <Card>
        <DataContainer {...state} data={track}>
          {(item) => (
            <>
              <ItemHeader item={item.item} />
              <CardContent>
                <TagChips
                  tags={item.tags}
                  deleteTag={(tag) => deleteTag({ uri, tag })}
                >
                  <NewTagChip
                    color="primary"
                    status={createTagStatus}
                    createTag={(tag) => createTag({ uri, tag })}
                  />
                </TagChips>
              </CardContent>
            </>
          )}
        </DataContainer>
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
