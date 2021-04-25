import React from "react";
import { makeStyles } from "@material-ui/core";

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
  readonly item: {
    readonly name: string;
    readonly images: readonly {
      readonly url: string;
    }[];
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
