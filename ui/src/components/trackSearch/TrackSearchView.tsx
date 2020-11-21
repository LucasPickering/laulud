import React, { useState } from "react";
import { makeStyles, Paper } from "@material-ui/core";
import SearchBar from "components/generic/SearchBar";

import TrackSearchList from "./TrackSearchList";
import useDebouncedValue from "hooks/useDebouncedValue";

const useStyles = makeStyles(({ spacing }) => ({
  container: {
    padding: spacing(1),
    width: 400,
  },
  searchBar: {
    width: "100%",
  },
}));

const TrackSearchView: React.FC = () => {
  const classes = useStyles();
  const [query, setQuery] = useState<string>("");
  const debouncedQuery = useDebouncedValue(query, 1000);

  return (
    <Paper className={classes.container}>
      <SearchBar
        className={classes.searchBar}
        value={query}
        onChange={(e) => setQuery(e.target.value)}
      />

      <TrackSearchList query={debouncedQuery} />
    </Paper>
  );
};

export default TrackSearchView;
