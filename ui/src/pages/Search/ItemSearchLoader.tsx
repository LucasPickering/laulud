import React, { useState } from "react";
import ItemList from "components/ItemList";
import { graphql, useLazyLoadQuery } from "react-relay";
import { ItemSearchLoaderQuery } from "./__generated__/ItemSearchLoaderQuery.graphql";
import withSuspense from "util/withSuspense";
import { Tab, Tabs } from "@mui/material";

interface Props
  extends Omit<
    React.ComponentProps<typeof ItemList>,
    "taggedItemConnectionKey"
  > {
  searchQuery: string;
}

/**
 * Results for an item search. Given a search query, this will execute the
 * search and render the results.
 */
const ItemSearchLoader: React.FC<Props> = ({ searchQuery, ...rest }) => {
  const data = useLazyLoadQuery<ItemSearchLoaderQuery>(
    graphql`
      query ItemSearchLoaderQuery(
        $searchQuery: String!
        $skipSearch: Boolean!
      ) {
        itemSearch(query: $searchQuery) @skip(if: $skipSearch) {
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
      }
    `,
    {
      searchQuery,
      skipSearch: !searchQuery,
    }
  );
  const [selectedTab, setSelectedTab] = useState<
    "tracks" | "albums" | "artists"
  >("tracks");

  if (!data.itemSearch) {
    return null;
  }

  return (
    <>
      <Tabs
        value={selectedTab}
        variant="fullWidth"
        onChange={(e, newSelectedTab) => setSelectedTab(newSelectedTab)}
      >
        <Tab value="tracks" label="Tracks" />
        <Tab value="albums" label="Albums" />
        <Tab value="artists" label="Artists" />
      </Tabs>
      <ItemList
        taggedItemConnectionKey={data.itemSearch[selectedTab]}
        {...rest}
      />
    </>
  );
};

export default withSuspense(ItemSearchLoader);
