import React from "react";
import { makeStyles } from "@material-ui/core";
import clsx from "clsx";
import TagChip from "./TagChip";
import { graphql, useFragment } from "react-relay";
import NewTagChip from "./NewTagChip";
import { TagChips_taggedItemNode$key } from "./__generated__/TagChips_taggedItemNode.graphql";
import { TagChipsDeleteTagMutation } from "./__generated__/TagChipsDeleteTagMutation.graphql";
import { TagChipsAddTagMutation } from "./__generated__/TagChipsAddTagMutation.graphql";
import ErrorSnackbar from "./generic/ErrorSnackbar";
import useMutation from "hooks/useMutation";

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
  const {
    commit: deleteTag,
    status: deleteTagStatus,
    resetStatus: resetDeleteTagStatus,
  } = useMutation<TagChipsDeleteTagMutation>(graphql`
    mutation TagChipsDeleteTagMutation($input: DeleteTagInput!) {
      deleteTag(input: $input) {
        # Grab this data so relay can update it in the store
        itemEdge {
          node {
            ...TagChips_taggedItemNode
          }
        }
        # TODO if this is the last tagged item, the tag doesn't get removed
        # from the list in the other tab
        tagEdge {
          node {
            ...TagDetails_tagNode
            ...TagList_tagNode
          }
        }
      }
    }
  `);
  const {
    commit: addTag,
    status: addTagStatus,
    resetStatus: resetAddTagStatus,
  } = useMutation<TagChipsAddTagMutation>(graphql`
    mutation TagChipsAddTagMutation($input: AddTagInput!) {
      addTag(input: $input) {
        # Grab this data so relay can update it in the store
        itemEdge {
          node {
            ...TagChips_taggedItemNode
          }
        }
        # TODO new tags don't get added to the tag list in the other tab
        tagEdge {
          node {
            ...TagDetails_tagNode
            ...TagList_tagNode
          }
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
          status={addTagStatus}
          addTag={(tag) =>
            addTag({
              variables: {
                input: { itemUri, tag },
              },
            })
          }
        />
      )}

      {/* Errors! */}
      <ErrorSnackbar
        message="Error deleting tag"
        status={deleteTagStatus}
        resetStatus={resetDeleteTagStatus}
      />
      <ErrorSnackbar
        message="Error adding tag"
        status={addTagStatus}
        resetStatus={resetAddTagStatus}
      />
    </div>
  );
};

export default TagChips;
