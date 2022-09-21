import React, { useEffect, useState } from "react";
import { Paper } from "@mui/material";
import { makeStyles } from "@mui/styles";
import queryString from "query-string";
import SearchBar from "components/generic/SearchBar";
import useRouteQuery from "hooks/useRouteQuery";
import { useNavigate } from "react-router-dom";
import ItemList from "components/ItemList";
import ItemSearchLoader from "./ItemSearchLoader";

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
  const classes = useStyles();
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
    <Paper className={classes.container}>
      <SearchBar
        className={classes.searchBar}
        initialQuery={(q ?? "").toString()}
        placeholder="Search tracks, albums, and artists…"
        onSearch={setSearchQuery}
      />

      <ItemSearchLoader searchQuery={searchQuery} {...rest} />
    </Paper>
  );
};

export default ItemSearchView;
