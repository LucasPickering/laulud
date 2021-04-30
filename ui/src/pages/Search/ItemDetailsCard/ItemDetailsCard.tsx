import React, { Suspense } from "react";
import { Card, CardContent, Typography } from "@material-ui/core";
import { graphql, useLazyLoadQuery } from "react-relay";
import ItemDetails from "./ItemDetails";
import { ItemDetailsCardQuery } from "./__generated__/ItemDetailsCardQuery.graphql";
import Loading from "components/Loading";

interface Props {
  uri: string;
}

/**
 * The main component to render details for a particular item. This handles
 * all the data loading, loading state, etc. needed
 */
const ItemDetailsCard: React.FC<Props> = ({ uri }) => (
  <Card>
    <Suspense
      fallback={
        <CardContent>
          <Loading />
        </CardContent>
      }
    >
      <ItemDetailsCardLoader uri={uri} />
    </Suspense>
  </Card>
);

const ItemDetailsCardLoader: React.FC<Props> = ({ uri }) => {
  const data = useLazyLoadQuery<ItemDetailsCardQuery>(
    graphql`
      query ItemDetailsCardQuery($uri: String!) {
        item(uri: $uri) {
          ...ItemDetails_taggedItemNode
        }
      }
    `,
    {
      uri,
    }
  );

  // URI doesn't match any item
  if (!data.item) {
    return <Typography>No such item</Typography>;
  }

  return <ItemDetails taggedItemNodeKey={data.item} />;
};

export default ItemDetailsCard;
