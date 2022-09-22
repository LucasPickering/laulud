import React, { useEffect, useState } from "react";
import { Search as SearchIcon } from "@mui/icons-material";
import { InputAdornment, TextField } from "@mui/material";

import useDebouncedValue from "hooks/useDebouncedValue";

interface Props {
  className?: string;
  initialQuery?: string;
  placeholder?: string;
  onSearch: (query: string) => void;
}

const SearchBar: React.FC<Props> = ({
  className,
  initialQuery,
  placeholder = "Search…",
  onSearch,
}) => {
  const [query, setQuery] = useState<string>(initialQuery ?? "");
  const debouncedQuery = useDebouncedValue(query, 500);

  useEffect(() => {
    onSearch(debouncedQuery);
  }, [onSearch, debouncedQuery]);

  return (
    <form
      className={className}
      onSubmit={(e) => {
        e.preventDefault(); // No page refresh
        onSearch(query);
      }}
    >
      <TextField
        placeholder={placeholder}
        value={query}
        variant="standard"
        InputProps={{
          "aria-label": "search",
          startAdornment: (
            <InputAdornment position="start">
              <SearchIcon />
            </InputAdornment>
          ),
        }}
        onChange={(e) => setQuery(e.target.value)}
        sx={{ width: "100%" }}
      />
    </form>
  );
};

export default SearchBar;
