import React, { useEffect, useState } from "react";
import { makeStyles, Paper } from "@material-ui/core";
import queryString from "query-string";
import SearchBar from "components/generic/SearchBar";
import DataContainer from "components/generic/DataContainer";
import { useQuery } from "react-query";
import { TaggedTrack } from "schema";
import { queryFn } from "util/queryCache";
import useRouteQuery from "hooks/useRouteQuery";
import { useHistory } from "react-router-dom";
import TrackList from "components/TrackList";

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
  query: string;
  setQuery: (query: string) => void;
  selectedTrackId?: string;
}

const TrackSearchList: React.FC<Props> = ({
  query,
  setQuery,
  selectedTrackId,
}) => {
  const classes = useStyles();
  const history = useHistory();
  const { q } = useRouteQuery();
  const state = useQuery<TaggedTrack[]>(
    ["tracks", { query }],
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
          <TrackList
            tracks={tracks}
            selectedTrackId={selectedTrackId}
            showTags
            routeMapper={(track) => ({
              ...history.location,
              pathname: `/tracks/${track.track.id}`,
            })}
          />
        )}
      </DataContainer>
    </Paper>
  );
};

export default TrackSearchList;
