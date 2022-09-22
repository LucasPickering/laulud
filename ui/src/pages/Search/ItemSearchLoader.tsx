import React from "react";
import ItemList from "components/ItemList";
import { graphql, useFragment } from "react-relay";
import { ItemSearchLoaderQuery } from "./__generated__/ItemSearchLoaderQuery.graphql";
import withQuery from "util/withQuery";
import { ItemSearchLoader_itemSearch$key } from "./__generated__/ItemSearchLoader_itemSearch.graphql";
import Loading from "components/Loading";

interface Props
  extends Omit<
    React.ComponentProps<typeof ItemList>,
    "taggedItemConnectionKey"
  > {
  itemSearchKey: ItemSearchLoader_itemSearch$key;
  selectedTab: "tracks" | "albums" | "artists";
}

/**
 * Results for an item search. Given a search query, this will execute the
 * search and render the results.
 *
 * TODO update comment
 */
const ItemSearchLoader: React.FC<Props> = ({
  itemSearchKey,
  selectedTab,
  ...rest
}) => {
  const itemSearch = useFragment(
    graphql`
      fragment ItemSearchLoader_itemSearch on ItemSearch {
        tracks {
          ...ItemList_taggedItemConnection
        }
        albums {
          ...ItemList_taggedItemConnection
        }
        artists {
          ...ItemList_taggedItemConnection
        }
      }
    `,
    itemSearchKey
  );

  return (
    <>
      <ItemList taggedItemConnectionKey={itemSearch[selectedTab]} {...rest} />
    </>
  );
};

export default withQuery<ItemSearchLoaderQuery, Props, "itemSearchKey">({
  query: graphql`
    query ItemSearchLoaderQuery($searchQuery: String!) {
      itemSearch(query: $searchQuery) {
        ...ItemSearchLoader_itemSearch
      }
    }
  `,
  dataToProps: (data) => data.itemSearch && { itemSearchKey: data.itemSearch },
  fallbackElement: <Loading margin={2} />,
})(ItemSearchLoader);
