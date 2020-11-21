import React, { useEffect, useState } from "react";
import { Search as SearchIcon } from "@material-ui/icons";
import { InputBase, makeStyles } from "@material-ui/core";
import clsx from "clsx";

import useDebouncedValue from "hooks/useDebouncedValue";

const useStyles = makeStyles(({ palette, shape, spacing }) => ({
  search: {
    position: "relative", // Needed because the icon is absolute
    display: "flex",
    alignItems: "center",
    borderRadius: shape.borderRadius,
    backgroundColor: palette.grey[800],
    "&:hover": {
      backgroundColor: palette.grey[700],
    },
  },
  searchIcon: {
    padding: spacing(0, 2),
    height: "100%",
    position: "absolute",
    pointerEvents: "none",
    display: "flex",
    alignItems: "center",
    justifyContent: "center",
  },
  inputRoot: {
    color: "inherit",
    width: "100%",
  },
  inputInput: {
    padding: spacing(1),
    // horizontal padding + font size from searchIcon
    paddingLeft: `calc(1em + ${spacing(4)})`,
    width: "100%",
  },
}));

interface Props {
  className?: string;
  onSearch: (query: string) => void;
}

const SearchBar: React.FC<Props> = ({ className, onSearch }) => {
  const classes = useStyles();
  const [query, setQuery] = useState<string>("");
  const debouncedQuery = useDebouncedValue(query, 500);

  useEffect(() => {
    onSearch(debouncedQuery);
  }, [onSearch, debouncedQuery]);

  return (
    <form
      className={clsx(classes.search, className)}
      onSubmit={(e) => {
        e.preventDefault(); // No page refresh
        onSearch(query);
      }}
    >
      <div className={classes.searchIcon}>
        <SearchIcon />
      </div>
      <InputBase
        placeholder="Search…"
        classes={{
          root: classes.inputRoot,
          input: classes.inputInput,
        }}
        inputProps={{ "aria-label": "search" }}
        value={query}
        onChange={(e) => setQuery(e.target.value)}
      />
    </form>
  );
};

export default SearchBar;
