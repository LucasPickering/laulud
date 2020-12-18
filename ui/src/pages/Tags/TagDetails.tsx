import React from "react";
import { makeStyles, Paper } from "@material-ui/core";
import DataContainer from "components/generic/DataContainer";
import { useQuery } from "react-query";
import { TagDetails as SchemaTagDetails } from "schema";
import { queryFn } from "util/queryCache";
import TrackList from "components/TrackList";

const useStyles = makeStyles(({ spacing }) => ({
  container: {
    padding: spacing(1),
  },
  searchBar: {
    width: "100%",
  },
  listItem: {
    flexWrap: "wrap",
  },
  listItemAvatar: {
    marginRight: spacing(2),
  },
  listItemTags: {
    flexBasis: "100%",
  },
}));

interface Props {
  tag: string;
}

const TagDetails: React.FC<Props> = ({ tag }) => {
  const classes = useStyles();
  const state = useQuery<SchemaTagDetails>(["tags", tag], () =>
    queryFn<SchemaTagDetails>({ url: `/api/tags/${tag}` })
  );

  return (
    <Paper className={classes.container}>
      <DataContainer {...state}>
        {(tagDetails) => <TrackList tracks={tagDetails.tracks} />}
      </DataContainer>
    </Paper>
  );
};

export default TagDetails;
