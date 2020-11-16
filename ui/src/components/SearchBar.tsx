import React, { useState } from "react";
import { Search as SearchIcon } from "@material-ui/icons";
import { IconButton, InputBase } from "@material-ui/core";

const SearchBar: React.FC = () => {
  // const localClasses = useLocalStyles();
  const [query, setQuery] = useState<string>("");

  return (
    <div>
      <InputBase
        // className={classes.input}
        placeholder="Search"
      />
      <IconButton aria-label="search">
        <SearchIcon />
      </IconButton>
    </div>
  );
};

export default SearchBar;
