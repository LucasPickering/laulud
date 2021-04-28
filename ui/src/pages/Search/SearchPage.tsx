import React, { useState } from "react";
import { useHistory, useParams } from "react-router-dom";
import { Grid } from "@material-ui/core";
import ItemSearchList from "./ItemSearchList";
import ItemDetails from "./ItemDetails";
import { graphql, useLazyLoadQuery } from "react-relay";
import { SearchPageQuery } from "./__generated__/SearchPageQuery.graphql";

interface RouteParams {
  selectedUri?: string;
}

const SearchPage: React.FC = () => {
  const { selectedUri } = useParams<RouteParams>();
  const [searchQuery, setSearchQuery] = useState<string>("asdf");
  const history = useHistory();
  const data = useLazyLoadQuery<SearchPageQuery>(
    graphql`
      query SearchPageQuery($query: String!, $skip: Boolean!) {
        itemSearch(query: $query) @skip(if: $skip) {
          ...ItemSearchList_itemSearch
        }
      }
    `,
    { query: searchQuery, skip: !searchQuery }
  );

  return (
    <Grid container spacing={2}>
      <Grid item xs={12} sm={6} md={4}>
        <ItemSearchList
          selectedUri={selectedUri}
          searchQuery={searchQuery}
          setSearchQuery={setSearchQuery}
          itemSearchKey={data.itemSearch}
          mapRoute={(uri) => ({
            ...history.location,
            pathname: `/search/${uri}`,
          })}
        />
      </Grid>
      {/* {selectedUri && (
        <Grid item xs={12} sm={6} md={8}>
          <ItemDetails
            taggedItemNodeKey={itemSearch}
          />
        </Grid>
      )} */}
    </Grid>
  );
};

export default SearchPage;
