import React, { useState } from "react";
import { Search as SearchIcon } from "@material-ui/icons";
import { IconButton, InputBase } from "@material-ui/core";

interface Props {
  onSearch?: (query: string) => void;
}

const SearchBar: React.FC<Props> = ({ onSearch }) => {
  // const localClasses = useLocalStyles();
  const [query, setQuery] = useState<string>("");

  return (
    <form
      onSubmit={(e) => {
        e.preventDefault(); // Don't reload the page
        if (onSearch) {
          onSearch(query);
        }
      }}
    >
      <InputBase
        // className={classes.input}
        required
        placeholder="Search"
        value={query}
        onChange={(e) => setQuery(e.target.value)}
      />
      <IconButton type="submit" aria-label="search">
        <SearchIcon />
      </IconButton>
    </form>
  );
};

export default SearchBar;
