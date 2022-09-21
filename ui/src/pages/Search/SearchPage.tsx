import React from "react";
import { useLocation, useParams } from "react-router-dom";
import { Grid } from "@mui/material";
import ItemSearchView from "./ItemSearchView";
import ItemDetailsCard from "./ItemDetailsCard/ItemDetailsCard";

interface RouteParams {
  selectedUri?: string;
}

const SearchPage: React.FC = () => {
  const { selectedUri } = useParams() as RouteParams;
  const location = useLocation();

  return (
    <Grid container spacing={2}>
      <Grid item xs={12} sm={6} md={4}>
        <ItemSearchView
          selectedUri={selectedUri}
          mapRoute={(uri) => ({
            ...location, // Retain query params
            pathname: `/search/${uri}`,
          })}
          showTags
        />
      </Grid>
      {selectedUri && (
        <Grid item xs={12} sm={6} md={8}>
          <ItemDetailsCard uri={selectedUri} />
        </Grid>
      )}
    </Grid>
  );
};

export default SearchPage;
