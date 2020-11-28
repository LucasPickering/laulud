import React from "react";
import { makeStyles } from "@material-ui/core";

import { AlbumSimplified } from "schema";

const useStyles = makeStyles(() => ({
  small: {
    width: 48,
    height: 48,
  },
  medium: {
    width: 96,
    height: 96,
  },
  large: {
    // TODO
  },
}));

interface Props {
  album: AlbumSimplified;
  size?: "small" | "medium" | "large";
}

function AlbumArt({ album, size = "medium" }: Props): React.ReactElement {
  const classes = useStyles();

  return (
    <img
      className={classes[size]}
      alt={`${album.name} album art`}
      src={album.images[0].url}
    />
  );
}

export default AlbumArt;
