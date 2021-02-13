import React from "react";
import { QueryKey, QueryStatus, useMutation, useQuery } from "react-query";
import {
  Alert,
  Card,
  CardContent,
  CardHeader,
  Snackbar,
} from "@material-ui/core";
import { Item, ItemSearchResponse, SpotifyUri, TaggedItem } from "schema";
import ItemArt from "components/generic/ItemArt";
import DataContainer from "components/generic/DataContainer";
import NewTagChip from "../../components/NewTagChip";
import queryCache, { queryFn } from "util/queryCache";
import TagChips from "components/TagChips";
import SpotifyLink from "components/generic/SpotifyLink";

function getQueryKey(uri: SpotifyUri): QueryKey {
  return ["items", { item: { uri } }];
}

function updateCachedTrack(
  itemListQueryKey: QueryKey | undefined,
  item: TaggedItem
): void {
  const uri = item.item.data.uri;
  const queryKey = getQueryKey(uri);
  queryCache.setQueryData<TaggedItem>(queryKey, item);
  if (itemListQueryKey) {
    queryCache.setQueryData<ItemSearchResponse>(itemListQueryKey, (data) => {
      // If we have a list of items cached, make sure we update the item there too
      if (data) {
        // Check each field in the cached search response (tracks/albums/artists)
        // and try to find the mutated item.
        [data.tracks, data.albums, data.artists].forEach((items) => {
          // Replace the item in the array
          const index = items.findIndex(
            (cachedItem) => cachedItem.item.data.uri === uri
          );
          if (index !== undefined) {
            items[index] = item;
          }
        });
        return data;
      }
      return { tracks: [], albums: [], artists: [] };
    });
  }
}

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
  itemListQueryKey?: QueryKey;
}> = ({ uri, itemListQueryKey }) => {
  const queryKey = getQueryKey(uri);
  const { data: track, ...state } = useQuery(queryKey, () =>
    queryFn<TaggedItem | undefined>({ url: `/api/items/${uri}` })
  );
  const [
    createTag,
    { status: createTagStatus, reset: resetCreateTagStatus },
  ] = useMutation(
    (tag: string) =>
      queryFn<TaggedItem>({
        url: `/api/items/${uri}/tags`,
        method: "POST",
        data: { tag },
      }),
    { onSuccess: (data) => updateCachedTrack(itemListQueryKey, data) }
  );
  const [deleteTag] = useMutation(
    (tag: string) =>
      queryFn<TaggedItem>({
        url: `/api/items/${uri}/tags/${encodeURIComponent(tag)}`,
        method: "DELETE",
      }),
    { onSuccess: (data) => updateCachedTrack(itemListQueryKey, data) }
  );

  return (
    <>
      <Card>
        <DataContainer {...state} data={track}>
          {(item) => (
            <>
              <ItemHeader item={item.item} />
              <CardContent>
                <TagChips tags={item.tags} deleteTag={deleteTag}>
                  <NewTagChip
                    color="primary"
                    status={createTagStatus}
                    createTag={createTag}
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
