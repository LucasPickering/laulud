import React, { useEffect, useState } from "react";
import { Paper } from "@mui/material";
import queryString from "query-string";
import SearchBar from "components/generic/SearchBar";
import useRouteQuery from "hooks/useRouteQuery";
import { useNavigate } from "react-router-dom";
import ItemList from "components/ItemList";
import ItemSearchLoader from "./ItemSearchLoader";

type Props = Omit<
  React.ComponentProps<typeof ItemList>,
  "taggedItemConnectionKey"
>;

/**
 * A searchable list of items. If itemSearchKey isn't provided, we assume no
 * search has been made yet, so the search bar will be rendered but results
 * won't.
 */
const ItemSearchView: React.FC<Props> = ({ ...rest }) => {
  const navigate = useNavigate();

  // Initialize the search based on the URL param
  const { q } = useRouteQuery();
  const [searchQuery, setSearchQuery] = useState<string>(q?.toString() ?? "");

  // Whenever the search changes, update the URL
  useEffect(() => {
    navigate(
      {
        search: queryString.stringify({ q: searchQuery }),
      },
      { replace: true }
    );
  }, [navigate, searchQuery]);

  return (
    <Paper>
      <SearchBar
        initialQuery={(q ?? "").toString()}
        placeholder="Search tracks, albums, and artists…"
        onSearch={setSearchQuery}
      />

      <ItemSearchLoader searchQuery={searchQuery} {...rest} />
    </Paper>
  );
};

export default ItemSearchView;
