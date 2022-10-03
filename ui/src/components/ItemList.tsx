import React from "react";
import { Box, List, ListItem, ListItemIcon, Stack } from "@mui/material";
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
  showIcon?: boolean;
  showLink?: boolean;
  showTags?: boolean;
  mapAction?: (uri: string, nodeId: string) => React.ReactNode;
  mapRoute?: (uri: string, nodeId: string) => To;
  onSelect?: (uri: string, nodeId: string) => void;
}

/**
 * A list of items (track/album/artist), where each item can be selected.
 */
const ItemList: React.FC<Props> = ({
  taggedItemConnectionKey,
  selectedUri,
  showIcon = false,
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
            id
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
        const nodeId = node.id;
        const action = mapAction && mapAction(uri, nodeId);

        // Render as a button if we have a link or onSelect
        // The typing on ListItem is really shitty so this has to be super jank
        const buttonProps: Record<string, unknown> = {};
        if (onSelect || mapRoute) {
          buttonProps.button = true;
          buttonProps.selected = uri === selectedUri;

          if (onSelect) {
            buttonProps.onClick = () => onSelect(uri, nodeId);
          }
          if (mapRoute) {
            buttonProps.component = UnstyledLink;
            buttonProps.to = mapRoute(uri, nodeId);
          }
        }

        return (
          <ListItem
            key={uri.toString()}
            secondaryAction={
              <>
                {showLink && <SpotifyLink itemKey={node.item} />}
                {action}
              </>
            }
            // Make sure tags render under the main content (icon/art/name/etc.)
            sx={{ flexDirection: "column" }}
            {...buttonProps}
          >
            <Stack direction="row" width="100%" alignItems="center">
              {showIcon && (
                <ListItemIcon>
                  <ItemIcon itemKey={node.item} />
                </ListItemIcon>
              )}
              <ItemListEntry taggedItemNodeKey={node} />
            </Stack>

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
