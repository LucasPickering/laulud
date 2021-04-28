import React, { useEffect, useState } from "react";
import { makeStyles, Paper, Tab, Tabs } from "@material-ui/core";
import queryString from "query-string";
import SearchBar from "components/generic/SearchBar";
import useRouteQuery from "hooks/useRouteQuery";
import { useHistory } from "react-router-dom";
import ItemList from "components/ItemList";
import { graphql, useFragment } from "react-relay";
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

interface Props
  extends Omit<
    React.ComponentProps<typeof ItemList>,
    "taggedItemConnectionKey"
  > {
  itemSearchKey?: ItemSearchList_itemSearch$key;
  searchQuery: string;
  setSearchQuery: (query: string) => void;
}

/**
 * A searchable list of items. If itemSearchKey isn't provided, we assume no
 * search has been made yet, so the search bar will be rendered but results
 * won't.
 */
const ItemSearchList: React.FC<Props> = ({
  itemSearchKey,
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
        }
        albums {
          ...ItemList_taggedItemConnection
        }
        artists {
          ...ItemList_taggedItemConnection
        }
      }
    `,
    itemSearchKey ?? null
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

      {itemSearch && (
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
            taggedItemConnectionKey={itemSearch[selectedTab]}
            showTags
            {...rest}
          />
        </>
      )}
    </Paper>
  );
};

export default ItemSearchList;
