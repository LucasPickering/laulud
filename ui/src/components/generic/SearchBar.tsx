import React from "react";
import { Search as SearchIcon } from "@material-ui/icons";
import { InputBase, makeStyles } from "@material-ui/core";
import clsx from "clsx";

const useStyles = makeStyles(({ palette, shape, spacing }) => ({
  search: {
    position: "relative", // Needed because the icon is absolute
    display: "flex",
    alignItems: "center",
    borderRadius: shape.borderRadius,
    backgroundColor: palette.grey[800],
    "&:hover": {
      backgroundColor: palette.grey[700],
    },
  },
  searchIcon: {
    padding: spacing(0, 2),
    height: "100%",
    position: "absolute",
    pointerEvents: "none",
    display: "flex",
    alignItems: "center",
    justifyContent: "center",
  },
  inputRoot: {
    color: "inherit",
    width: "100%",
  },
  inputInput: {
    padding: spacing(1),
    // horizontal padding + font size from searchIcon
    paddingLeft: `calc(1em + ${spacing(4)})`,
    width: "100%",
  },
}));

interface Props {
  className?: string;
  value?: string;
  onChange?: (
    e: React.ChangeEvent<HTMLTextAreaElement | HTMLInputElement>
  ) => void;
}

const SearchBar: React.FC<Props> = ({ className, value, onChange }) => {
  const classes = useStyles();

  return (
    <div className={clsx(classes.search, className)}>
      <div className={classes.searchIcon}>
        <SearchIcon />
      </div>
      <InputBase
        placeholder="Search…"
        classes={{
          root: classes.inputRoot,
          input: classes.inputInput,
        }}
        inputProps={{ "aria-label": "search" }}
        value={value}
        onChange={onChange}
      />
    </div>
  );
};

export default SearchBar;
