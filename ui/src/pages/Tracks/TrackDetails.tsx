import React from "react";
import { QueryStatus, useMutation, useQuery } from "react-query";
import {
  Alert,
  Card,
  CardContent,
  CardHeader,
  Chip,
  makeStyles,
  Snackbar,
} from "@material-ui/core";

import { TaggedTrack } from "util/schema";
import AlbumArt from "components/generic/AlbumArt";
import DataContainer from "components/generic/DataContainer";
import NewTagChip from "./NewTagChip";
import queryCache, { queryFn } from "util/queryCache";

const useStyles = makeStyles(({ spacing }) => ({
  tags: {
    display: "flex",
    // Spacing between children
    "& > * + *": {
      marginLeft: spacing(1),
    },
  },
}));

interface Props {
  trackId: string;
}

const TrackDetails: React.FC<Props> = ({ trackId }) => {
  const classes = useStyles();
  const queryKey = `track-${trackId}`;
  const { data: track, ...state } = useQuery<TaggedTrack | undefined>(
    queryKey,
    () => queryFn<TaggedTrack | undefined>({ url: `/api/tracks/${trackId}` })
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
    { onSuccess: (data) => queryCache.setQueryData(queryKey, data) }
  );
  const [deleteTag] = useMutation(
    (tag: string) =>
      queryFn<TaggedTrack>({
        url: `/api/tracks/${trackId}/tags/${tag}`,
        method: "DELETE",
      }),
    { onSuccess: (data) => queryCache.setQueryData(queryKey, data) }
  );

  return (
    <>
      <DataContainer {...state} data={track}>
        {(track) => (
          <Card>
            <CardHeader
              title={track.track.name}
              subheader={track.track.artists
                .map((artist) => artist.name)
                .join(", ")}
              avatar={<AlbumArt album={track.track.album} />}
            />
            <CardContent>
              <div className={classes.tags}>
                {track.tags.map((tag) => (
                  <Chip
                    key={tag}
                    label={tag}
                    color="primary"
                    onDelete={() => deleteTag(tag)}
                  />
                ))}
                <NewTagChip
                  color="primary"
                  status={createTagStatus}
                  createTag={createTag}
                />
              </div>
            </CardContent>
          </Card>
        )}
      </DataContainer>
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
