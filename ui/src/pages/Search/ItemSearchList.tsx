import React, { useEffect, useState } from "react";
import { makeStyles, Paper, Tab, Tabs } from "@material-ui/core";
import queryString from "query-string";
import SearchBar from "components/generic/SearchBar";
import useRouteQuery from "hooks/useRouteQuery";
import { useHistory } from "react-router-dom";
import ItemList from "components/ItemList";
import { useFragment } from "react-relay";
import { ItemSearchList_itemSearch$key } from "./__generated__/ItemSearchList_itemSearch.graphql";

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
  itemSearchNodeKey: ItemSearchList_itemSearch$key;
  searchQuery: string;
  setSearchQuery: (query: string) => void;
}

const ItemSearchList: React.FC<Props> = ({
  itemSearchNodeKey,
  searchQuery,
  setSearchQuery,
  ...rest
}) => {
  const classes = useStyles();
  const history = useHistory();
  const { q } = useRouteQuery();
  const [selectedTab, setSelectedTab] = useState<
    "tracks" | "albums" | "artists"
  >("tracks");
  const itemSearch = useFragment(
    graphql`
      fragment ItemSearchList_itemSearch on ItemSearch {
        tracks {
          ...ItemList_taggedItemConnection
          edges {
            node {
              ...ItemDetails_itemNode
            }
          }
        }
      }
    `,
    itemSearchNodeKey
  );

  // Whenever the search changes, update the URL
  useEffect(() => {
    history.replace({
      ...history.location,
      search: queryString.stringify({ q: searchQuery }),
    });
  }, [history, searchQuery]);

  return (
    <Paper className={classes.container}>
      <SearchBar
        className={classes.searchBar}
        initialQuery={(q ?? "").toString()}
        placeholder="Search tracks, albums, and artists…"
        onSearch={setSearchQuery}
      />

      <Tabs
        classes={{ root: classes.tabs }}
        value={selectedTab}
        variant="fullWidth"
        onChange={(e, newSelectedTab) => setSelectedTab(newSelectedTab)}
      >
        <Tab classes={{ root: classes.tab }} value="tracks" label="Tracks" />
        <Tab classes={{ root: classes.tab }} value="albums" label="Albums" />
        <Tab classes={{ root: classes.tab }} value="artists" label="Artists" />
      </Tabs>
      <ItemList items={getListItems(data, selectedTab)} showTags {...rest} />
    </Paper>
  );
};

export default ItemSearchList;
