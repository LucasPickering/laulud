import React, { useEffect, useState } from "react";
import {
  List,
  ListItem,
  ListItemAvatar,
  ListItemText,
  makeStyles,
  Paper,
} from "@material-ui/core";
import queryString from "query-string";
import SearchBar from "components/generic/SearchBar";
import AlbumArt from "components/generic/AlbumArt";
import DataContainer from "components/generic/DataContainer";
import { useQuery } from "react-query";
import { TaggedTrack } from "util/schema";
import UnstyledLink from "components/generic/UnstyledLink";
import TagChips from "./TagChips";
import { queryFn } from "util/queryCache";
import useRouteQuery from "hooks/useRouteQuery";
import { useHistory } from "react-router-dom";

const useStyles = makeStyles(({ spacing }) => ({
  container: {
    padding: spacing(1),
  },
  searchBar: {
    width: "100%",
  },
  listItem: {
    flexWrap: "wrap",
  },
  listItemAvatar: {
    marginRight: spacing(2),
  },
  listItemTags: {
    flexBasis: "100%",
  },
}));

interface Props {
  selectedTrackId?: string;
}

const TrackSearchList: React.FC<Props> = ({ selectedTrackId }) => {
  const classes = useStyles();
  const history = useHistory();
  const { q } = useRouteQuery();
  const [query, setQuery] = useState<string>("");
  const state = useQuery<TaggedTrack[]>(
    "tracks",
    () => queryFn({ url: `/api/tracks/search/${query}` }),
    { enabled: Boolean(query) }
  );

  // Whenever the search changes, update the URL
  useEffect(() => {
    history.replace({
      ...history.location,
      search: queryString.stringify({ q: query }),
    });
  }, [history, query]);

  return (
    <Paper className={classes.container}>
      <SearchBar
        className={classes.searchBar}
        initialQuery={(q ?? "").toString()}
        onSearch={setQuery}
      />

      <DataContainer {...state}>
        {(tracks) => (
          <List>
            {tracks.map((track) => (
              <ListItem
                key={track.track.id}
                className={classes.listItem}
                button
                selected={track.track.id === selectedTrackId}
                component={UnstyledLink}
                to={{
                  ...history.location,
                  pathname: `/tracks/${track.track.id}`,
                }}
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
                <TagChips className={classes.listItemTags} tags={track.tags} />
              </ListItem>
            ))}
          </List>
        )}
      </DataContainer>
    </Paper>
  );
};

export default TrackSearchList;
