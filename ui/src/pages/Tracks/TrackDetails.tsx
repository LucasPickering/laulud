import React from "react";
import {
  Card,
  CardContent,
  CardHeader,
  Chip,
  makeStyles,
} from "@material-ui/core";

import { TaggedTrack } from "util/schema";
import AlbumArt from "components/generic/AlbumArt";

const useStyles = makeStyles(({ spacing }) => ({}));

interface Props {
  track: TaggedTrack;
}

const TrackDetails: React.FC<Props> = ({ track }) => {
  const classes = useStyles();

  return (
    <Card>
      <CardHeader
        title={track.track.name}
        subheader={track.track.artists.map((artist) => artist.name).join(", ")}
        avatar={<AlbumArt album={track.track.album} />}
      />
      <CardContent>
        {track.tags.map((tag) => (
          <Chip key={tag} label={tag} />
        ))}
      </CardContent>
    </Card>
  );
};

export default TrackDetails;
