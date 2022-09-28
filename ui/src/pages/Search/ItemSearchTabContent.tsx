import React from "react";
import ItemList from "components/ItemList";
import { graphql, useFragment } from "react-relay";
import { ItemSearchTabContentQuery } from "./__generated__/ItemSearchTabContentQuery.graphql";
import withQuery from "util/withQuery";
import { ItemSearchTabContent_itemSearch$key } from "./__generated__/ItemSearchTabContent_itemSearch.graphql";
import Loading from "components/Loading";

interface Props
  extends Omit<
    React.ComponentProps<typeof ItemList>,
    "taggedItemConnectionKey"
  > {
  itemSearchKey: ItemSearchTabContent_itemSearch$key;
  selectedTab: "tracks" | "albums" | "artists";
}

/**
 * A single selected tab in the item search view.
 */
const ItemSearchTabContent: React.FC<Props> = ({
  itemSearchKey,
  selectedTab,
  ...rest
}) => {
  const itemSearch = useFragment(
    graphql`
      fragment ItemSearchTabContent_itemSearch on ItemSearch {
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

export default withQuery<ItemSearchTabContentQuery, Props, "itemSearchKey">({
  query: graphql`
    query ItemSearchTabContentQuery($searchQuery: String!) {
      itemSearch(query: $searchQuery) {
        ...ItemSearchTabContent_itemSearch
      }
    }
  `,
  dataToProps: (data) => data.itemSearch && { itemSearchKey: data.itemSearch },
  // Lists are ugly af w/ skeleton, so stick to loading icon for now
  fallbackElement: <Loading margin={2} />,
})(ItemSearchTabContent);
