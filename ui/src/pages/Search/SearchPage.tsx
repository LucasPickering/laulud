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
      query SearchPageQuery(
        $searchQuery: String!
        $skipSearch: Boolean!
        $selectedUri: String!
        $skipItem: Boolean!
      ) {
        itemSearch(query: $searchQuery) @skip(if: $skipSearch) {
          ...ItemSearchList_itemSearch
        }
        item(uri: $selectedUri) @skip(if: $skipItem) {
          ...ItemDetails_taggedItemNode
        }
      }
    `,
    {
      searchQuery,
      skipSearch: !searchQuery,
      selectedUri: selectedUri ?? "",
      skipItem: !selectedUri,
    }
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
      {data.item && (
        <Grid item xs={12} sm={6} md={8}>
          <ItemDetails taggedItemNodeKey={data.item} />
        </Grid>
      )}
    </Grid>
  );
};

export default SearchPage;
