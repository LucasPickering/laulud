import React from "react";
import { makeStyles } from "@material-ui/core";
import clsx from "clsx";
import TagChip from "./TagChip";

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
        <TagChip
          key={tag}
          className={classes.tag}
          tag={tag}
          onDelete={deleteTag && (() => deleteTag(tag))}
        />
      ))}
      {children}
    </div>
  );
};

export default TagChips;
