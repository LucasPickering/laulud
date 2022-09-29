import React from "react";
import { Box, List, ListItem, ListItemIcon } from "@mui/material";
import UnstyledLink from "components/generic/UnstyledLink";
import TagChips from "./TagChips";
import ItemIcon from "./generic/ItemIcon";
import SpotifyLink from "./generic/SpotifyLink";
import { graphql, useFragment } from "react-relay";
import { ItemList_taggedItemConnection$key } from "./__generated__/ItemList_taggedItemConnection.graphql";
import ItemListEntry from "./ItemListEntry";
import { To } from "react-router-dom";

interface Props {
  taggedItemConnectionKey: ItemList_taggedItemConnection$key;
  selectedUri?: string;
  showLink?: boolean;
  showTags?: boolean;
  mapAction?: (uri: string) => React.ReactNode;
  mapRoute?: (uri: string) => To;
  onSelect?: (uri: string) => void;
}

/**
 * A list of items (track/album/artist), where each item can be selected.
 */
const ItemList: React.FC<Props> = ({
  taggedItemConnectionKey,
  selectedUri,
  showLink = false,
  showTags = false,
  mapAction,
  mapRoute,
  onSelect,
}) => {
  const taggedItemConnection = useFragment(
    graphql`
      fragment ItemList_taggedItemConnection on TaggedItemConnection {
        edges {
          node {
            item {
              uri
              ...ItemIcon_item
              ...SpotifyLink_item
            }
            ...ItemListEntry_taggedItemNode
            ...TagChips_taggedItemNode
          }
        }
      }
    `,
    taggedItemConnectionKey
  );

  return (
    <List>
      {taggedItemConnection.edges.map(({ node }) => {
        const uri = node.item.uri;
        const action = mapAction && mapAction(uri);

        // Render as a button if we have a link or onSelect
        // The typing on ListItem is really shitty so this has to be super jank
        const buttonProps: Record<string, unknown> = {};
        if (onSelect || mapRoute) {
          buttonProps.button = true;
          buttonProps.selected = uri === selectedUri;

          if (onSelect) {
            buttonProps.onClick = () => onSelect(uri);
          }
          if (mapRoute) {
            buttonProps.component = UnstyledLink;
            buttonProps.to = mapRoute(uri);
          }
        }

        return (
          <ListItem
            key={uri.toString()}
            // Wrapping makes tags render correctly
            sx={{ flexWrap: "wrap" }}
            secondaryAction={
              <>
                {showLink && <SpotifyLink itemKey={node.item} />}
                {action}
              </>
            }
            {...buttonProps}
          >
            <ListItemIcon>
              <ItemIcon itemKey={node.item} />
            </ListItemIcon>
            <ItemListEntry taggedItemNodeKey={node} />

            {showTags && (
              <Box flexBasis="100%">
                <TagChips taggedItemNodeKey={node} />
              </Box>
            )}
          </ListItem>
        );
      })}
    </List>
  );
};

export default ItemList;
