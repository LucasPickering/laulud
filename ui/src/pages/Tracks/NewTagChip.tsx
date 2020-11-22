import React, { useState } from "react";
import { Chip, InputBase, makeStyles } from "@material-ui/core";
import { Add as AddIcon } from "@material-ui/icons";
import { QueryStatus } from "react-query";
import DataContainer from "components/generic/DataContainer";

const useStyles = makeStyles(({ spacing }) => ({
  tags: {
    display: "flex",
    // Spacing between children
    "& > * + *": {
      marginLeft: spacing(1),
    },
  },
}));

interface Props extends React.ComponentProps<typeof Chip> {
  status: QueryStatus;
  createTag: (tagName: string) => void;
}

const NewTagChip: React.FC<Props> = ({ status, createTag, ...rest }) => {
  const classes = useStyles();
  const [isEditing, setIsEditing] = useState<boolean>(false);
  const [newTagText, setNewTagText] = useState<string>("");

  return (
    <Chip
      {...rest}
      icon={<AddIcon />}
      label={
        <DataContainer
          status={status}
          data={undefined}
          idleEl={
            <form
              onSubmit={(e) => {
                e.preventDefault();
                createTag(newTagText);
              }}
            >
              <InputBase
                value={newTagText}
                onChange={(e) => setNewTagText(e.target.value)}
              />
            </form>
          }
        />
      }
      clickable
      onClick={() => setIsEditing(true)}
    />
  );
};

export default NewTagChip;
