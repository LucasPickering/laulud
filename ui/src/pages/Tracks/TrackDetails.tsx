import React from "react";
import { QueryKey, QueryStatus, useMutation, useQuery } from "react-query";
import {
  Alert,
  Card,
  CardContent,
  CardHeader,
  Snackbar,
} from "@material-ui/core";

import { TaggedTrack } from "schema";
import AlbumArt from "components/generic/AlbumArt";
import DataContainer from "components/generic/DataContainer";
import NewTagChip from "../../components/NewTagChip";
import queryCache, { queryFn } from "util/queryCache";
import TagChips from "components/TagChips";

interface Props {
  trackId: string;
}

function getQueryKey(trackId: string): QueryKey {
  return ["tracks", { track: { id: trackId } }];
}

function updateCachedTrack(track: TaggedTrack): void {
  const trackId = track.track.id;
  const queryKey = getQueryKey(trackId);
  queryCache.setQueryData<TaggedTrack>(queryKey, track);
  queryCache.setQueryData<TaggedTrack[]>("tracks", (tracks) => {
    // If we have a list of tracks cached, make sure we update the track there too
    if (tracks) {
      const index = tracks.findIndex(
        (cachedTrack) => cachedTrack.track.id === trackId
      );
      if (index !== undefined) {
        tracks[index] = track;
      }
      return tracks;
    }
    return [];
  });
}

const TrackDetails: React.FC<Props> = ({ trackId }) => {
  const queryKey = ["tracks", { track: { id: trackId } }];
  const { data: track, ...state } = useQuery(
    queryKey,
    () => queryFn<TaggedTrack | undefined>({ url: `/api/tracks/${trackId}` }),
    {
      // Grab initial data from the search list
      initialData: () =>
        queryCache
          .getQueryData<TaggedTrack[]>("tracks")
          ?.find((track) => track.track.id === trackId),
    }
  );
  const [
    createTag,
    { status: createTagStatus, reset: resetCreateTagStatus },
  ] = useMutation(
    (tag: string) =>
      queryFn<TaggedTrack>({
        url: `/api/tracks/${trackId}/tags`,
        method: "POST",
        data: { tag },
      }),
    { onSuccess: (data) => updateCachedTrack(data) }
  );
  const [deleteTag] = useMutation(
    (tag: string) =>
      queryFn<TaggedTrack>({
        url: `/api/tracks/${trackId}/tags/${tag}`,
        method: "DELETE",
      }),
    { onSuccess: (data) => updateCachedTrack(data) }
  );

  return (
    <>
      <Card>
        <DataContainer {...state} data={track}>
          {(track) => (
            <>
              <CardHeader
                title={track.track.name}
                subheader={track.track.artists
                  .map((artist) => artist.name)
                  .join(", ")}
                avatar={<AlbumArt album={track.track.album} />}
              />
              <CardContent>
                <TagChips tags={track.tags} deleteTag={deleteTag}>
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

export default TrackDetails;
