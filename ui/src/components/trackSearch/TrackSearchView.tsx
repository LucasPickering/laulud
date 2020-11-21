import React, { useState } from "react";
import { makeStyles, Paper } from "@material-ui/core";
import SearchBar from "components/generic/SearchBar";

import TrackSearchList from "./TrackSearchList";

const useStyles = makeStyles(({ spacing }) => ({
  container: {
    padding: spacing(1),
    width: 400,
  },
  searchBar: {
    width: "100%",
  },
}));

const TrackSearchContainer: React.FC = () => {
  const classes = useStyles();
  const [query, setQuery] = useState<string>("");

  return (
    <Paper className={classes.container}>
      <SearchBar className={classes.searchBar} onSearch={setQuery} />

      {query && <TrackSearchList query={query} />}
    </Paper>
  );
};

export default TrackSearchContainer;
