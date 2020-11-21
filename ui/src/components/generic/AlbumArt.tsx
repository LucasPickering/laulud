import React from "react";
import { makeStyles } from "@material-ui/core";

import { AlbumSimplified } from "util/schema";

const useStyles = makeStyles(() => ({
  small: {
    width: 64,
    height: 64,
  },
  medium: {
    // TODO
  },
  large: {
    // TODO
  },
}));

interface Props {
  album: AlbumSimplified;
  size: "small" | "medium" | "large";
}

const AlbumArt: React.FC<Props> = ({ album, size }) => {
  const classes = useStyles();

  return (
    <img
      className={classes[size]}
      alt={`${album.name} album art`}
      src={album.images[0].url}
    />
  );
};

export default AlbumArt;
