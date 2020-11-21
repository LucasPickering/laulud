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

import useDebouncedValue from "hooks/useDebouncedValue";
import AlbumArt from "components/generic/AlbumArt";
import DataContainer from "components/generic/DataContainer";
import { useQuery } from "react-query";
import { TaggedTrack } from "util/schema";

const useStyles = makeStyles(({ spacing }) => ({
  container: {
    padding: spacing(1),
    width: 400,
  },
  searchBar: {
    width: "100%",
  },
  listItemAvatar: {
    marginRight: spacing(2),
  },
}));

const TrackSearchView: React.FC = () => {
  const classes = useStyles();
  const [query, setQuery] = useState<string>("");
  const debouncedQuery = useDebouncedValue(query, 1000);
  const state = useQuery<TaggedTrack[]>(
    `/api/tracks/search/${debouncedQuery}`,
    { enabled: Boolean(debouncedQuery) }
  );

  return (
    <Paper className={classes.container}>
      <SearchBar
        className={classes.searchBar}
        value={query}
        onChange={(e) => setQuery(e.target.value)}
      />

      <DataContainer {...state}>
        {(tracks) => (
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
        )}
      </DataContainer>
    </Paper>
  );
};

export default TrackSearchView;
