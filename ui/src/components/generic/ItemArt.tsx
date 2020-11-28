import React from "react";
import { makeStyles } from "@material-ui/core";

import { Image } from "schema";

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
  item: {
    name: string;
    images: Image[];
  };
  size?: "small" | "medium" | "large";
}

function ItemArt({ item, size = "medium" }: Props): React.ReactElement {
  const classes = useStyles();

  return (
    <img
      className={classes[size]}
      alt={`${item.name} icon`}
      src={item.images[0]?.url}
    />
  );
}

export default ItemArt;
