import React, { useEffect, useState } from "react";
import { makeStyles, Paper, Tab, Tabs } from "@material-ui/core";
import queryString from "query-string";
import SearchBar from "components/generic/SearchBar";
import DataContainer from "components/generic/DataContainer";
import { ItemSearchResponse, TaggedItem } from "schema";
import useRouteQuery from "hooks/useRouteQuery";
import { useHistory } from "react-router-dom";
import ItemList from "components/ItemList";
import useLauludQuery from "hooks/useLauludQuery";

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

interface Props extends Omit<React.ComponentProps<typeof ItemList>, "items"> {
  query: string;
  setQuery: (query: string) => void;
}

const ItemSearchList: React.FC<Props> = ({ query, setQuery, ...rest }) => {
  const classes = useStyles();
  const history = useHistory();
  const { q } = useRouteQuery();
  const [selectedTab, setSelectedTab] = useState<
    "tracks" | "albums" | "artists"
  >("tracks");

  const state = useLauludQuery(["items", "search", query], undefined, {
    enabled: Boolean(query),
  });

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
        placeholder="Search tracks, albums, and artists…"
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
              showTags
              {...rest}
            />
          </>
        )}
      </DataContainer>
    </Paper>
  );
};

export default ItemSearchList;
