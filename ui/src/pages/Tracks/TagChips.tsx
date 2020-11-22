import React from "react";
import { Chip, makeStyles } from "@material-ui/core";
import clsx from "clsx";

const useStyles = makeStyles(({ spacing }) => ({
  tags: {
    display: "flex",
    flexWrap: "wrap",
  },
  tag: {
    margin: spacing(0.5),
  },
}));

interface Props {
  className?: string;
  tags: string[];
  deleteTag?: (tag: string) => void;
}

const TagChips: React.FC<Props> = ({
  className,
  tags,
  deleteTag,
  children,
}) => {
  const classes = useStyles();

  return (
    <div className={clsx(classes.tags, className)}>
      {tags.map((tag) => (
        <Chip
          key={tag}
          className={classes.tag}
          label={tag}
          color="primary"
          onDelete={deleteTag && (() => deleteTag(tag))}
        />
      ))}
      {children}
    </div>
  );
};

export default TagChips;
