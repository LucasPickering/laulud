import React from "react";
import { useHistory } from "react-router-dom";

import SearchBar from "./generic/SearchBar";

const TrackSearchBar: React.FC = () => {
  // const localClasses = useLocalStyles();
  const history = useHistory();

  return (
    <SearchBar
      onSearch={(query) => {
        history.push(`/tracks/search/${query}`);
      }}
    />
  );
};

export default TrackSearchBar;
