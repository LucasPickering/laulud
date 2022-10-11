import React from "react";
import ItemList from "components/ItemList";
import { graphql, useFragment } from "react-relay";
import { withQuery } from "relay-query-wrapper";
import Loading from "components/Loading";
import { TagDetailsItemListQuery } from "./__generated__/TagDetailsItemListQuery.graphql";
import { TagDetailsItemList_tagNode$key } from "./__generated__/TagDetailsItemList_tagNode.graphql";
import { IconButton, Tooltip } from "@mui/material";
import { Clear as IconClear } from "@mui/icons-material";
import ErrorSnackbar from "components/generic/ErrorSnackbar";
import useMutation from "hooks/useMutation";
import { TagDetailsItemListDeleteTagMutation } from "./__generated__/TagDetailsItemListDeleteTagMutation.graphql";

interface Props {
  tagNodeKey: TagDetailsItemList_tagNode$key;
}

/**
 * The list of items that a tag has been applied to, within the scope of the
 * TagDetails component.
 */
const TagDetailsItemList: React.FC<Props> = ({ tagNodeKey }) => {
  const tagNode = useFragment(
    graphql`
      fragment TagDetailsItemList_tagNode on TagNode {
        tag
        items {
          __id
          ...ItemList_taggedItemConnection
        }
      }
    `,
    tagNodeKey
  );

  const {
    commit: deleteTag,
    status: deleteTagStatus,
    resetStatus: resetDeleteTagStatus,
  } = useMutation<TagDetailsItemListDeleteTagMutation>(graphql`
    mutation TagDetailsItemListDeleteTagMutation(
      $input: DeleteTagInput!
      $connections: [ID!]!
    ) {
      deleteTag(input: $input) {
        itemEdge {
          node {
            id @deleteEdge(connections: $connections)
          }
        }
      }
    }
  `);

  return (
    <>
      <ItemList
        taggedItemConnectionKey={tagNode.items}
        showIcon
        showLink
        mapAction={(uri, nodeId) => (
          <Tooltip title="Remove tag">
            <IconButton
              onClick={() => {
                deleteTag({
                  variables: {
                    input: { itemUri: uri, tag: tagNode.tag },
                    connections: [tagNode.items.__id],
                  },
                  optimisticResponse: {
                    deleteTag: { itemEdge: { node: { id: nodeId } } },
                  },
                });
              }}
            >
              <IconClear />
            </IconButton>
          </Tooltip>
        )}
      />
      <ErrorSnackbar
        message="Error removing tag"
        status={deleteTagStatus}
        resetStatus={resetDeleteTagStatus}
      />
    </>
  );
};

export default withQuery<TagDetailsItemListQuery, Props, "tagNodeKey">({
  query: graphql`
    query TagDetailsItemListQuery($tag: Tag!) {
      tag(tag: $tag) {
        ...TagDetailsItemList_tagNode
      }
    }
  `,
  dataToProps: (data) => data.tag && { tagNodeKey: data.tag },
  // Lists are ugly af w/ skeleton, so stick to loading icon for now
  fallbackElement: <Loading />,
})(TagDetailsItemList);
