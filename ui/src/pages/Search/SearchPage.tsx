import React, { useState } from "react";
import { useHistory, useParams } from "react-router-dom";
import { Grid } from "@material-ui/core";
import ItemSearchView from "./ItemSearchView";
import ItemDetails from "./ItemDetails";
import { graphql, useLazyLoadQuery } from "react-relay";
import { SearchPageQuery } from "./__generated__/SearchPageQuery.graphql";

interface RouteParams {
  selectedUri?: string;
}

const SearchPage: React.FC = () => {
  const { selectedUri } = useParams<RouteParams>();
  const history = useHistory();
  const data = useLazyLoadQuery<SearchPageQuery>(
    graphql`
      query SearchPageQuery($selectedUri: String!, $skipItem: Boolean!) {
        item(uri: $selectedUri) @skip(if: $skipItem) {
          ...ItemDetails_taggedItemNode
        }
      }
    `,
    {
      selectedUri: selectedUri ?? "",
      skipItem: !selectedUri,
    }
  );

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
      {data.item && (
        <Grid item xs={12} sm={6} md={8}>
          <ItemDetails taggedItemNodeKey={data.item} />
        </Grid>
      )}
    </Grid>
  );
};

export default SearchPage;
