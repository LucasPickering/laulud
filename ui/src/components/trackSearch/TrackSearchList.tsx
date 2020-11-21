import React from "react";
import {
  Avatar,
  CircularProgress,
  List,
  ListItem,
  ListItemText,
} from "@material-ui/core";
import { useQuery } from "react-query";

import { Track } from "api/types";

interface Props {
  query: string;
}

const TrackSearchList: React.FC<Props> = ({ query }) => {
  const { isLoading, data: tracks } = useQuery<Track[]>(
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
        <ListItem key={track.track.id}>
          <ListItemAvatar>{/* <Avatar alt={`${track}`} /> */}</ListItemAvatar>
          <ListItemText primary={track.track.name} />
        </ListItem>
      ))}
    </List>
  );
};

export default TrackSearchList;
