import React, { useEffect, useState } from "react";
import { makeStyles, Paper, Tab, Tabs } from "@material-ui/core";
import queryString from "query-string";
import SearchBar from "components/generic/SearchBar";
import DataContainer from "components/generic/DataContainer";
import { QueryKey, useQuery } from "react-query";
import { ItemSearchResponse, SpotifyUri, TaggedItem } from "schema";
import { queryFn } from "util/queryCache";
import useRouteQuery from "hooks/useRouteQuery";
import { useHistory } from "react-router-dom";
import ItemList from "components/ItemList";

const useStyles = makeStyles(({ spacing }) => ({
  container: {
    padding: spacing(1),
  },
  searchBar: {
    width: "100%",
  },
  tabs: {
    minHeight: "unset",
  },
  tab: {
    minWidth: 80,
    minHeight: 36,
    textTransform: "none",
  },
}));

export function getItemSearchQueryKey(query: string): QueryKey {
  return ["itemSearch", { query }];
}

function getListItems(
  data: ItemSearchResponse,
  selectedTab: "tracks" | "albums" | "artists"
): TaggedItem[] {
  switch (selectedTab) {
    case "tracks":
      return data.tracks;
    case "albums":
      return data.albums;
    case "artists":
      return data.artists;
  }
}

interface Props {
  selectedUri?: SpotifyUri;
  query: string;
  setQuery: (query: string) => void;
}

const ItemSearchList: React.FC<Props> = ({ selectedUri, query, setQuery }) => {
  const classes = useStyles();
  const history = useHistory();
  const { q } = useRouteQuery();
  const [selectedTab, setSelectedTab] = useState<
    "tracks" | "albums" | "artists"
  >("tracks");

  const state = useQuery<ItemSearchResponse>(
    getItemSearchQueryKey(query),
    () => queryFn({ url: `/api/items/search/${query}` }),
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
        {(data) => (
          <>
            <Tabs
              classes={{ root: classes.tabs }}
              value={selectedTab}
              variant="fullWidth"
              onChange={(e, newSelectedTab) => setSelectedTab(newSelectedTab)}
            >
              <Tab
                classes={{ root: classes.tab }}
                value="tracks"
                label="Tracks"
              />
              <Tab
                classes={{ root: classes.tab }}
                value="albums"
                label="Albums"
              />
              <Tab
                classes={{ root: classes.tab }}
                value="artists"
                label="Artists"
              />
            </Tabs>
            <ItemList
              items={getListItems(data, selectedTab)}
              selectedUri={selectedUri}
              showTags
              mapRoute={(item) => ({
                ...history.location,
                pathname: `/search/${item.data.uri}`,
              })}
            />
          </>
        )}
      </DataContainer>
    </Paper>
  );
};

export default ItemSearchList;
