import React from "react";
import { Search as SearchIcon } from "@material-ui/icons";
import { fade, InputBase, makeStyles } from "@material-ui/core";
import clsx from "clsx";

const useStyles = makeStyles(
  ({ breakpoints, palette, shape, spacing, transitions }) => ({
    search: {
      position: "relative",
      borderRadius: shape.borderRadius,
      backgroundColor: fade(palette.common.white, 0.15),
      "&:hover": {
        backgroundColor: fade(palette.common.white, 0.25),
      },
      marginRight: spacing(2),
      marginLeft: 0,
      width: "100%",
      [breakpoints.up("sm")]: {
        marginLeft: spacing(3),
        width: "auto",
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
    },
    inputInput: {
      padding: spacing(1, 1, 1, 0),
      // vertical padding + font size from searchIcon
      paddingLeft: `calc(1em + ${spacing(4)}px)`,
      transition: transitions.create("width"),
      width: "100%",
      [breakpoints.up("md")]: {
        width: "20ch",
      },
    },
  })
);
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
    // <div className={className}>
    //   <InputBase
    //     required
    //     placeholder="Search"
    //     value={value}
    //     onChange={onChange}
    //   />
    //   <SearchIcon />
    // </div>
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
