import React, { useEffect } from "react";
import { Card } from "@mui/material";
import { useQueryLoader } from "react-relay";
import ItemDetails from "./ItemDetails";
import type { ItemDetailsQuery as ItemDetailsQueryType } from "./__generated__/ItemDetailsQuery.graphql";
import ItemDetailsQuery from "./__generated__/ItemDetailsQuery.graphql";

interface Props {
  uri: string;
}

/**
 * The main component to render details for a particular item. This handles
 * all the data loading needed
 */
const ItemDetailsCard: React.FC<Props> = ({ uri }) => {
  const [queryRef, loadQuery] =
    useQueryLoader<ItemDetailsQueryType>(ItemDetailsQuery);

  // Kick off query on first load
  useEffect(() => {
    loadQuery({ uri });
  }, [loadQuery, uri]);

  return (
    <Card>
      <ItemDetails queryRef={queryRef} />
    </Card>
  );
};

export default ItemDetailsCard;
