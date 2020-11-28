import React, { useState } from "react";
import { useParams } from "react-router-dom";
import { Grid } from "@material-ui/core";
import ItemSearchList, { getItemSearchQueryKey } from "./ItemSearchList";
import ItemDetails from "./ItemDetails";

interface RouteParams {
  selectedUri?: string;
}

const SearchPage: React.FC = () => {
  const { selectedUri } = useParams<RouteParams>();
  const [query, setQuery] = useState<string>("");

  return (
    <Grid container spacing={2}>
      <Grid item xs={12} sm={6} md={4}>
        <ItemSearchList
          selectedUri={selectedUri}
          query={query}
          setQuery={setQuery}
        />
      </Grid>
      {selectedUri && (
        <Grid item xs={12} sm={6} md={8}>
          <ItemDetails
            uri={selectedUri}
            itemListQueryKey={getItemSearchQueryKey(query)}
          />
        </Grid>
      )}
    </Grid>
  );
};

export default SearchPage;
