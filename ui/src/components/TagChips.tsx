import React from "react";
import { makeStyles } from "@material-ui/core";
import clsx from "clsx";
import TagChip from "./TagChip";
import { useFragment } from "react-relay";
import { TagChips_itemNode$key } from "./__generated__/TagChips_itemNode.graphql";

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
  itemNodeKey: TagChips_itemNode$key;
  showAdd?: boolean;
  showDelete?: boolean;
}

/**
 * Show a list of tags for an item, with optional add+delete buttons
 */
const TagChips: React.FC<Props> = ({
  className,
  itemNodeKey,
  showAdd = false,
  showDelete = false,
  children,
}) => {
  const classes = useStyles();
  const itemNode = useFragment(
    graphql`
      fragment TagChips_itemNode on TaggedItemNode {
        tags {
          edges {
            node {
              tag
            }
          }
        }
      }
    `,
    itemNodeKey
  );

  return (
    <div className={clsx(classes.tags, className)}>
      {itemNode.tags.edges.map(({ node: { tag } }) => (
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
