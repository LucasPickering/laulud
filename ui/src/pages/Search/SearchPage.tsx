import React from "react";
import { useHistory, useParams } from "react-router-dom";
import { Grid } from "@material-ui/core";
import ItemSearchView from "./ItemSearchView";
import ItemDetailsCard from "./ItemDetailsCard/ItemDetailsCard";

interface RouteParams {
  selectedUri?: string;
}

const SearchPage: React.FC = () => {
  const { selectedUri } = useParams<RouteParams>();
  const history = useHistory();

  return (
    <Grid container spacing={2}>
      <Grid item xs={12} sm={6} md={4}>
        <ItemSearchView
          selectedUri={selectedUri}
          mapRoute={(uri) => ({
            ...history.location,
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
