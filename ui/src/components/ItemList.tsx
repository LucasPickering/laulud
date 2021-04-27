import React from "react";
import { List, ListItem, ListItemIcon, makeStyles } from "@material-ui/core";
import UnstyledLink from "components/generic/UnstyledLink";
import { LocationDescriptorObject } from "history";
import TagChips from "./TagChips";
import ItemIcon from "./generic/ItemIcon";
import SpotifyLink from "./generic/SpotifyLink";
import { graphql, useFragment } from "react-relay";
import { ItemList_taggedItemConnection$key } from "./__generated__/ItemList_taggedItemConnection.graphql";
import ItemListEntry from "./ItemListEntry";

const useStyles = makeStyles(({ spacing }) => ({
  listItem: {
    flexWrap: "wrap",
  },
  listItemAvatar: {
    marginRight: spacing(2),
  },
  listItemTags: {
    flexBasis: "100%",
  },
}));

interface Props {
  className?: string;
  taggedItemConnectionKey: ItemList_taggedItemConnection$key;
  selectedUri?: string;
  showIcons?: boolean;
  showTags?: boolean;
  mapAction?: (uri: string) => React.ReactNode;
  mapRoute?: (uri: string) => string | LocationDescriptorObject;
  onSelect?: (uri: string) => void;
}

/**
 * A list of items (track/album/artist), where each item can be selected.
 */
function ItemList({
  className,
  taggedItemConnectionKey,
  selectedUri,
  showIcons = false,
  showTags = false,
  mapAction,
  mapRoute,
  onSelect,
}: Props): React.ReactElement {
  const classes = useStyles();
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
    <List className={className}>
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
            className={classes.listItem}
            {...buttonProps}
          >
            <ItemListEntry taggedItemNodeKey={node} />
            {showIcons && (
              <>
                <ListItemIcon>
                  <ItemIcon itemKey={node.item} />
                </ListItemIcon>
                <ListItemIcon>
                  <SpotifyLink itemKey={node.item} />
                </ListItemIcon>
              </>
            )}
            {action}

            {showTags && (
              <TagChips
                className={classes.listItemTags}
                taggedItemNodeKey={node}
              />
            )}
          </ListItem>
        );
      })}
    </List>
  );
}

export default ItemList;
