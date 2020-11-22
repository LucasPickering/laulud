import React, { useState } from "react";
import {
  List,
  ListItem,
  ListItemAvatar,
  ListItemText,
  makeStyles,
  Paper,
} from "@material-ui/core";
import SearchBar from "components/generic/SearchBar";

import AlbumArt from "components/generic/AlbumArt";
import DataContainer from "components/generic/DataContainer";
import { useQuery } from "react-query";
import { TaggedTrack } from "util/schema";
import UnstyledLink from "components/generic/UnstyledLink";

const useStyles = makeStyles(({ spacing }) => ({
  container: {
    padding: spacing(1),
  },
  searchBar: {
    width: "100%",
  },
  listItemAvatar: {
    marginRight: spacing(2),
  },
}));

interface Props {
  selectedTrackId?: string;
}

const TrackSearchList: React.FC<Props> = ({ selectedTrackId }) => {
  const classes = useStyles();
  const [query, setQuery] = useState<string>("");
  const state = useQuery<TaggedTrack[]>(`/api/tracks/search/${query}`, {
    enabled: Boolean(query),
  });

  return (
    <Paper className={classes.container}>
      <SearchBar className={classes.searchBar} onSearch={setQuery} />

      <DataContainer {...state}>
        {(tracks) => (
          <List>
            {tracks.map((track) => (
              <ListItem
                key={track.track.id}
                button
                selected={track.track.id === selectedTrackId}
                component={UnstyledLink}
                to={`/tracks/${track.track.id}`}
              >
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
        )}
      </DataContainer>
    </Paper>
  );
};

export default TrackSearchList;
