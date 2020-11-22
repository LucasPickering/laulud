import React from "react";
import { useMutation, useQuery } from "react-query";
import {
  Card,
  CardContent,
  CardHeader,
  Chip,
  makeStyles,
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
  const [createTag, { status: createTagStatus }] = useMutation(
    (tag: string) =>
      queryFn<TaggedTrack>({
        url: `/api/tracks/${trackId}/tags`,
        method: "POST",
        data: { tags: [tag] },
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
                  color="secondary"
                  onDelete={() => deleteTag(tag)}
                />
              ))}
              <NewTagChip
                color="secondary"
                status={createTagStatus}
                createTag={createTag}
              />
            </div>
          </CardContent>
        </Card>
      )}
    </DataContainer>
  );
};

export default TrackDetails;
