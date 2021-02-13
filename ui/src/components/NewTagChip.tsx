import React, { useEffect, useState } from "react";
import {
  Chip,
  CircularProgress,
  InputBase,
  Tooltip,
  makeStyles,
} from "@material-ui/core";
import { Add as AddIcon } from "@material-ui/icons";
import { QueryStatus } from "react-query";

const useStyles = makeStyles(({ spacing }) => ({
  tag: {
    margin: spacing(0.5),
  },
  inputWrapper: {
    display: "flex",
    alignItems: "center",
  },
}));

interface Props extends React.ComponentProps<typeof Chip> {
  status: QueryStatus;
  createTag: (tag: string) => void;
}

const NewTagChip: React.FC<Props> = ({ status, createTag, ...rest }) => {
  const classes = useStyles();
  const [isEditing, setIsEditing] = useState<boolean>(false);
  const [newTagText, setNewTagText] = useState<string>("");

  // After successfully creating a tag, reset state here
  useEffect(() => {
    if (status === QueryStatus.Success) {
      setIsEditing(false);
      setNewTagText("");
    }
  }, [status]);

  return (
    <Tooltip title="Add Tag">
      <Chip
        {...rest}
        className={classes.tag}
        icon={
          status === QueryStatus.Loading ? (
            <CircularProgress color="secondary" size={24} />
          ) : (
            <AddIcon />
          )
        }
        label={
          <div className={classes.inputWrapper}>
            {isEditing ? (
              <form
                onSubmit={(e) => {
                  e.preventDefault();
                  createTag(newTagText);
                  setIsEditing(false);
                }}
              >
                <InputBase
                  autoFocus
                  value={newTagText}
                  onBlur={() => setIsEditing(false)}
                  onChange={(e) => setNewTagText(e.target.value)}
                />
              </form>
            ) : (
              newTagText || null
            )}
          </div>
        }
        clickable
        onClick={() => setIsEditing(true)}
      />
    </Tooltip>
  );
};

export default NewTagChip;
