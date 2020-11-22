import React from "react";
import { useMutation, useQuery } from "react-query";
import {
  Card,
  CardContent,
  CardHeader,
  Chip,
  makeStyles,
} from "@material-ui/core";
import axios from "axios";

import { TaggedTrack } from "util/schema";
import AlbumArt from "components/generic/AlbumArt";
import DataContainer from "components/generic/DataContainer";
import NewTagChip from "./NewTagChip";

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
  const { data: initialTrack, ...state } = useQuery<TaggedTrack | undefined>(
    `/api/tracks/${trackId}`
  );
  const [createTag, { status, data: mutatedTrack }] = useMutation(
    (tag: string) =>
      axios
        .post<TaggedTrack>(`/api/tracks/${trackId}/tags`, { tags: [tag] })
        .then((response) => response.data)
  );

  return (
    <DataContainer {...state} data={mutatedTrack ?? initialTrack}>
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
                <Chip key={tag} label={tag} />
              ))}
              <NewTagChip status={status} createTag={createTag} />
            </div>
          </CardContent>
        </Card>
      )}
    </DataContainer>
  );
};

export default TrackDetails;
