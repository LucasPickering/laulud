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
  items: TaggedItem[];
  selectedUri?: SpotifyUri;
  showIcons?: boolean;
  showTags?: boolean;
  mapRoute?: (item: Item) => string | LocationDescriptorObject;
  onSelect?: (uri: SpotifyUri) => void;
}

/**
 * A list of items (track/album/artist), where each item can be selected.
 */
function ItemList({
  className,
  items,
  selectedUri,
  showIcons = false,
  showTags = false,
  mapRoute,
  onSelect,
}: Props): React.ReactElement {
  const classes = useStyles();

  return (
    <List className={className}>
      {items.map((item) => {
        const uri = item.item.data.uri;
        return (
          <ListItem
            key={uri.toString()}
            className={classes.listItem}
            selected={uri === selectedUri}
            button
            // If a route mapper is given, use it to turn this item into a link
            {...(mapRoute && {
              component: UnstyledLink,
              to: mapRoute(item.item),
            })}
            onClick={onSelect && (() => onSelect(uri))}
          >
            <ItemListEntry item={item} />
            {showIcons && (
              <ListItemIcon>
                <ItemIcon item={item.item} />
              </ListItemIcon>
            )}

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
