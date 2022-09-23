import React, { useEffect, useState } from "react";
import { Paper, Tab, Tabs } from "@mui/material";
import queryString from "query-string";
import SearchBar from "components/generic/SearchBar";
import useRouteQuery from "hooks/useRouteQuery";
import { useNavigate } from "react-router-dom";
import type { ItemSearchLoaderQuery as ItemSearchLoaderQueryType } from "./__generated__/ItemSearchLoaderQuery.graphql";
import ItemSearchLoaderQuery from "./__generated__/ItemSearchLoaderQuery.graphql";
import ItemSearchLoader from "./ItemSearchLoader";
import { useQueryLoader } from "react-relay";

interface Props
  extends Omit<
    React.ComponentProps<typeof ItemSearchLoader>,
    "queryRef" | "selectedTab"
  > {
  persistInRoute?: boolean;
}

/**
 * A searchable list of items. If itemSearchKey isn't provided, we assume no
 * search has been made yet, so the search bar will be rendered but results
 * won't.
 * @param persistInRoute if true, the search query will be persisted in a route param
 */
const ItemSearchView: React.FC<Props> = ({
  persistInRoute = false,
  ...rest
}) => {
  const [queryRef, loadQuery, disposeQuery] =
    useQueryLoader<ItemSearchLoaderQueryType>(ItemSearchLoaderQuery);
  const navigate = useNavigate();
  const [selectedTab, setSelectedTab] = useState<
    "tracks" | "albums" | "artists"
  >("tracks");

  // Initialize the search based on the URL param
  const { q } = useRouteQuery();
  const initialQuery = persistInRoute ? q?.toString() ?? "" : "";
  const [searchQuery, setSearchQuery] = useState<string>(initialQuery);

  useEffect(() => {
    if (searchQuery) {
      loadQuery({ searchQuery });
    } else {
      // Search is empty, wipe out previous results
      disposeQuery();
    }
  }, [loadQuery, disposeQuery, searchQuery]);

  // Whenever the search changes, update the URL
  useEffect(() => {
    if (persistInRoute) {
      navigate(
        { search: queryString.stringify({ q: searchQuery }) },
        { replace: true }
      );
    }
  }, [navigate, persistInRoute, searchQuery]);

  return (
    <Paper>
      <SearchBar
        initialQuery={initialQuery}
        placeholder="Search tracks, albums, and artists…"
        onSearch={setSearchQuery}
      />

      {searchQuery && (
        <>
          <Tabs
            value={selectedTab}
            variant="fullWidth"
            onChange={(e, newSelectedTab) => setSelectedTab(newSelectedTab)}
          >
            <Tab value="tracks" label="Tracks" />
            <Tab value="albums" label="Albums" />
            <Tab value="artists" label="Artists" />
          </Tabs>
          <ItemSearchLoader
            queryRef={queryRef}
            selectedTab={selectedTab}
            {...rest}
          />
        </>
      )}
    </Paper>
  );
};

export default ItemSearchView;
