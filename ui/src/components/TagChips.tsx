import React from "react";
import { makeStyles } from "@material-ui/core";
import clsx from "clsx";
import TagChip from "./TagChip";
import { graphql, useFragment, useMutation } from "react-relay";
import NewTagChip from "./NewTagChip";
import { TagChips_taggedItemNode$key } from "./__generated__/TagChips_taggedItemNode.graphql";
import { TagChipsDeleteTagMutation } from "./__generated__/TagChipsDeleteTagMutation.graphql";
import { TagChipsAddTagMutation } from "./__generated__/TagChipsAddTagMutation.graphql";

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
}) => {
  const classes = useStyles();
  const taggedItemNode = useFragment(
    graphql`
      fragment TagChips_taggedItemNode on TaggedItemNode {
        item {
          uri
        }
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
  const [deleteTag] = useMutation<TagChipsDeleteTagMutation>(graphql`
    mutation TagChipsDeleteTagMutation($input: DeleteTagInput!) {
      deleteTag(input: $input) {
        item {
          ...TagChips_taggedItemNode
        }
      }
    }
  `);
  const [addTag, isAddInFlight] = useMutation<TagChipsAddTagMutation>(graphql`
    mutation TagChipsAddTagMutation($input: AddTagInput!) {
      addTag(input: $input) {
        item {
          ...TagChips_taggedItemNode
        }
      }
    }
  `);
  const itemUri = taggedItemNode.item.uri;

  return (
    <div className={clsx(classes.tags, className)}>
      {taggedItemNode.tags.edges.map(({ node: { tag } }) => (
        <TagChip
          key={tag}
          className={classes.tag}
          tag={tag}
          onDelete={
            showDelete
              ? () =>
                  deleteTag({
                    variables: {
                      input: { itemUri, tag },
                    },
                  })
              : undefined
          }
        />
      ))}

      {showAdd && (
        <NewTagChip
          color="primary"
          // TODO track idle/error here
          status={isAddInFlight ? "loading" : "success"}
          addTag={(tag) =>
            addTag({
              variables: {
                input: { itemUri, tag },
              },
            })
          }
        />
      )}
    </div>
  );
};

export default TagChips;
