import React from "react";
import {
  List,
  ListItem,
  ListItemAvatar,
  ListItemText,
  makeStyles,
} from "@material-ui/core";
import AlbumArt from "components/generic/AlbumArt";
import { TaggedTrack } from "schema";
import UnstyledLink from "components/generic/UnstyledLink";
import { LocationDescriptorObject } from "history";
import TagChips from "./TagChips";

const useStyles = makeStyles(({ spacing }) => ({
  container: {
    padding: spacing(1),
  },
  searchBar: {
    width: "100%",
  },
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
  tracks: TaggedTrack[];
  selectedTrackId?: string;
  showTags?: boolean;
  routeMapper?: (track: TaggedTrack) => string | LocationDescriptorObject;
  onSelect?: (trackId: string) => void;
}

function TrackList({
  className,
  tracks,
  selectedTrackId,
  showTags = false,
  routeMapper,
  onSelect,
}: Props): React.ReactElement {
  const classes = useStyles();

  return (
    <List className={className}>
      {tracks.map((track) => (
        <ListItem
          key={track.track.id}
          className={classes.listItem}
          selected={track.track.id === selectedTrackId}
          button
          // If a route mapper is given, use it to turn this item into a link
          {...(routeMapper && {
            component: UnstyledLink,
            to: routeMapper(track),
          })}
          onClick={onSelect && (() => onSelect(track.track.id))}
        >
          <ListItemAvatar className={classes.listItemAvatar}>
            <AlbumArt album={track.track.album} size="small" />
          </ListItemAvatar>
          <ListItemText
            primary={track.track.name}
            secondary={track.track.artists
              .map((artist) => artist.name)
              .join(", ")}
          />
          {showTags && (
            <TagChips className={classes.listItemTags} tags={track.tags} />
          )}
        </ListItem>
      ))}
    </List>
  );
}

export default TrackList;
