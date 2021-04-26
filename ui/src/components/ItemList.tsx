import React from "react";
import {
  List,
  ListItem,
  ListItemAvatar,
  ListItemIcon,
  ListItemText,
  makeStyles,
} from "@material-ui/core";
import UnstyledLink from "components/generic/UnstyledLink";
import { LocationDescriptorObject } from "history";
import { Item, SpotifyUri, TaggedItem } from "schema";
import ItemArt from "./generic/ItemArt";
import TagChips from "./TagChips";
import ItemIcon from "./generic/ItemIcon";
import SpotifyLinkIcon from "./generic/SpotifyLink";
import { useFragment } from "react-relay";
import { ItemList_taggedItemConnection$key } from "./__generated__/ItemList_taggedItemConnection.graphql";

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

function ItemListEntry({ item }: { item: TaggedItem }): React.ReactElement {
  const classes = useStyles();

  switch (item.item.type) {
    case "track":
      return (
        <>
          <ListItemAvatar className={classes.listItemAvatar}>
            <ItemArt item={item.item.data.album} size="small" />
          </ListItemAvatar>
          <ListItemText
            primary={item.item.data.name}
            secondary={item.item.data.artists
              .map((artist) => artist.name)
              .join(", ")}
          />
        </>
      );
    case "album":
      return (
        <>
          <ListItemAvatar className={classes.listItemAvatar}>
            <ItemArt item={item.item.data} size="small" />
          </ListItemAvatar>
          <ListItemText
            primary={item.item.data.name}
            secondary={item.item.data.artists
              .map((artist) => artist.name)
              .join(", ")}
          />
        </>
      );
    case "artist":
      return (
        <>
          <ListItemAvatar className={classes.listItemAvatar}>
            <ItemArt item={item.item.data} size="small" />
          </ListItemAvatar>
          <ListItemText primary={item.item.data.name} />
        </>
      );
  }
}

interface Props {
  className?: string;
  taggedItemConnectionNodeKey: ItemList_taggedItemConnection$key;
  selectedUri?: SpotifyUri;
  showIcons?: boolean;
  showTags?: boolean;
  mapAction?: (item: Item) => React.ReactNode;
  mapRoute?: (item: Item) => string | LocationDescriptorObject;
  onSelect?: (uri: SpotifyUri) => void;
}

/**
 * A list of items (track/album/artist), where each item can be selected.
 */
function ItemList({
  className,
  taggedItemConnectionNodeKey,
  selectedUri,
  showIcons = false,
  showTags = false,
  mapAction,
  mapRoute,
  onSelect,
}: Props): React.ReactElement {
  const classes = useStyles();
  const itemConnection = useFragment(
    graphql`
      fragment ItemList_taggedItemConnection on TaggedItemConnection {
        edges {
          node {
            item {
              __typename
              ... on Track {
                uri
              }
              ... on AlbumSimplified {
                uri
              }
              ... on Artist {
                uri
              }
            }
          }
        }
      }
    `,
    taggedItemConnectionNodeKey
  );

  return (
    <List className={className}>
      {itemConnection.edges.map(({ node }) => {
        const uri = node.item.uri;
        const action = mapAction && mapAction(item.item);

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
            buttonProps.to = mapRoute(item.item);
          }
        }

        return (
          <ListItem
            key={uri.toString()}
            className={classes.listItem}
            {...buttonProps}
          >
            <ItemListEntry item={item} />
            {showIcons && (
              <>
                <ListItemIcon>
                  <ItemIcon item={item.item} />
                </ListItemIcon>
                <ListItemIcon>
                  <SpotifyLinkIcon item={item.item} />
                </ListItemIcon>
              </>
            )}
            {action}

            {showTags && (
              <TagChips className={classes.listItemTags} tags={item.tags} />
            )}
          </ListItem>
        );
      })}
    </List>
  );
}

export default ItemList;
