import React, { useState } from "react";
import { useMutation } from "react-query";
import {
  Button,
  Card,
  CardContent,
  CardHeader,
  Chip,
  makeStyles,
  TextField,
} from "@material-ui/core";
import axios from "axios";

import { TaggedTrack } from "util/schema";
import AlbumArt from "components/generic/AlbumArt";

const useStyles = makeStyles(({ spacing }) => ({}));

interface Props {
  track: TaggedTrack;
}

const TrackDetails: React.FC<Props> = ({ track }) => {
  const classes = useStyles();
  const [newTagText, setNewTagText] = useState<string>();
  const [createTag, { status }] = useMutation(() =>
    axios.post(`/api/tracks/${track.track.id}/tags`, { tags: [newTagText] })
  );

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
        <form
          onSubmit={(e) => {
            e.preventDefault();
            createTag();
          }}
        >
          <TextField
            id="new-tag-input"
            label="New Tag"
            value={newTagText}
            onChange={(e) => setNewTagText(e.target.value)}
          />
          <Button type="submit">Add</Button>
        </form>
      </CardContent>
    </Card>
  );
};

export default TrackDetails;
