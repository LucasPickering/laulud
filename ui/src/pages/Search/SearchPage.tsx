import React, { useState } from "react";
import { useHistory, useParams } from "react-router-dom";
import { Grid } from "@material-ui/core";
import ItemSearchList from "./ItemSearchList";
import ItemDetails from "./ItemDetails";
import { useLazyLoadQuery } from "react-relay";
import { SearchPageQuery } from "./__generated__/SearchPageQuery.graphql";

interface RouteParams {
  selectedUri?: string;
}

const SearchPage: React.FC = () => {
  const { selectedUri } = useParams<RouteParams>();
  const [searchQuery, setSearchQuery] = useState<string>("");
  const history = useHistory();
  const itemSearch = useLazyLoadQuery<SearchPageQuery>(
    graphql`
      query SearchPageQuery($query: String!) {
        itemSearch(query: $query) {
          ...ItemSearchList_itemSearch
        }
      }
    `,
    { query: searchQuery }
  );

  return (
    <Grid container spacing={2}>
      <Grid item xs={12} sm={6} md={4}>
        <ItemSearchList
          selectedUri={selectedUri}
          searchQuery={searchQuery}
          setSearchQuery={setSearchQuery}
          mapRoute={(item) => ({
            ...history.location,
            pathname: `/search/${item.data.uri}`,
          })}
        />
      </Grid>
      {/* {selectedUri && (
        <Grid item xs={12} sm={6} md={8}>
          <ItemDetails
            itemNodeKey={itemSearch}
          />
        </Grid>
      )} */}
    </Grid>
  );
};

export default SearchPage;
