import React from "react";
import { Search as SearchIcon } from "@material-ui/icons";
import { IconButton, InputBase } from "@material-ui/core";

interface Props {
  className?: string;
  value?: string;
  onChange?: (
    e: React.ChangeEvent<HTMLTextAreaElement | HTMLInputElement>
  ) => void;
}

const SearchBar: React.FC<Props> = ({ className, value, onChange }) => {
  return (
    <div className={className}>
      <InputBase
        required
        placeholder="Search"
        value={value}
        onChange={onChange}
      />
      <IconButton type="submit" aria-label="search">
        <SearchIcon />
      </IconButton>
    </div>
  );
};

export default SearchBar;
