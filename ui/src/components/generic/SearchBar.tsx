import React, { useEffect, useState } from "react";
import { Search as SearchIcon } from "@mui/icons-material";
import { InputAdornment, TextField } from "@mui/material";

import useDebouncedValue from "hooks/useDebouncedValue";

interface Props {
  initialQuery?: string;
  placeholder?: string;
  debounceMs?: number;
  onSearch: (query: string) => void;
}

/**
 * Debounced search text input. The initial query will pre-populated the search
 * bar, then the setter will be called whenever the input value changes and
 * settles for some number of milliseconds.
 */
const SearchBar: React.FC<Props> = ({
  initialQuery,
  placeholder = "Search…",
  debounceMs = 500,
  onSearch,
}) => {
  // Internal state is updated immediately. We won't propagate to the parent
  // until after the debounce expires
  const [query, setQuery] = useState<string>(initialQuery ?? "");
  const debouncedQuery = useDebouncedValue(query, debounceMs);

  useEffect(() => {
    onSearch(debouncedQuery);
  }, [onSearch, debouncedQuery]);

  return (
    <form
      onSubmit={(e) => {
        e.preventDefault(); // No page refresh
        onSearch(query);
      }}
    >
      <TextField
        autoFocus
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
