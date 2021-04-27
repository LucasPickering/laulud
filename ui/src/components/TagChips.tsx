import React from "react";
import { makeStyles } from "@material-ui/core";
import clsx from "clsx";
import TagChip from "./TagChip";
import { useFragment } from "react-relay";
import { TagChips_taggedItemNode$key } from "./__generated__/TagChips_taggedItemNode.graphql";

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
  taggedItemNodeKey: TagChips_taggedItemNode$key;
  showAdd?: boolean;
  showDelete?: boolean;
}

/**
 * Show a list of tags for an item, with optional add+delete buttons
 */
const TagChips: React.FC<Props> = ({
  className,
  taggedItemNodeKey,
  showAdd = false,
  showDelete = false,
  children,
}) => {
  const classes = useStyles();
  const taggedItemNode = useFragment(
    graphql`
      fragment TagChips_taggedItemNode on TaggedItemNode {
        tags {
          edges {
            node {
              tag
            }
          }
        }
      }
    `,
    taggedItemNodeKey
  );

  return (
    <div className={clsx(classes.tags, className)}>
      {taggedItemNode.tags.edges.map(({ node: { tag } }) => (
        <TagChip
          key={tag}
          className={classes.tag}
          tag={tag}
          onDelete={showDelete && (() => deleteTag(tag))}
        />
      ))}
      {children}
    </div>
  );
};

export default TagChips;
