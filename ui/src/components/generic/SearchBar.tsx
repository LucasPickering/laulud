import React, { useState } from "react";
import { Search as SearchIcon } from "@material-ui/icons";
import { IconButton, InputBase } from "@material-ui/core";

interface Props {
  className?: string;
  onSearch?: (query: string) => void;
}

const SearchBar: React.FC<Props> = ({ className, onSearch }) => {
  const [query, setQuery] = useState<string>("");

  return (
    <form
      className={className}
      onSubmit={(e) => {
        e.preventDefault(); // Don't reload the page
        if (onSearch) {
          onSearch(query);
        }
      }}
    >
      <InputBase
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
