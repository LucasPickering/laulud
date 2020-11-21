import React from "react";
import {
  CircularProgress,
  List,
  ListItem,
  ListItemAvatar,
  ListItemText,
  makeStyles,
} from "@material-ui/core";
import { useQuery } from "react-query";

import { TaggedTrack } from "util/schema";
import AlbumArt from "components/generic/AlbumArt";

const useStyles = makeStyles(({ spacing }) => ({
  listItemAvatar: {
    marginRight: spacing(2),
  },
}));

interface Props {
  query: string;
}

const TrackSearchList: React.FC<Props> = ({ query }) => {
  const classes = useStyles();
  const { isLoading, data: tracks } = useQuery<TaggedTrack[]>(
    `/api/tracks/search/${query}`,
    { enabled: Boolean(query) }
  );

  if (isLoading) {
    return <CircularProgress />;
  }

  if (!tracks) {
    return <>shit</>;
  }

  return (
    <List>
      {tracks.map((track) => (
        <ListItem key={track.track.id} button>
          <ListItemAvatar className={classes.listItemAvatar}>
            <AlbumArt album={track.track.album} size="small" />
          </ListItemAvatar>
          <ListItemText
            primary={track.track.name}
            secondary={track.track.artists
              .map((artist) => artist.name)
              .join(", ")}
          />
        </ListItem>
      ))}
    </List>
  );
};

export default TrackSearchList;
