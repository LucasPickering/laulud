import { Theme, createTheme, responsiveFontSizes } from "@mui/material";
import { green, red } from "@mui/material/colors";

declare module "@mui/styles/defaultTheme" {
  // eslint-disable-next-line @typescript-eslint/no-empty-interface
  interface DefaultTheme extends Theme {}
}

function theme(): Theme {
  return responsiveFontSizes(
    createTheme({
      palette: {
        mode: "dark",
        primary: green,
        secondary: red,
        divider: "#ffffff",
        background: {
          default: "#000000",
          paper: "#212121",
        },
      },
      typography: {
        // Makes math for `rem` font sizes easy
        // https://www.sitepoint.com/understanding-and-using-rem-units-in-css/
        htmlFontSize: 10,

        h1: { fontSize: "3.2rem" },
        h2: { fontSize: "2.8rem" },
        h3: { fontSize: "2.4rem" },
        h4: { fontSize: "2.0rem" },
        h5: { fontSize: "1.6rem" },
        h6: { fontSize: "1.2rem" },
      },

      components: {
        MuiButton: {
          defaultProps: {
            color: "inherit",
          },
        },
        MuiLink: {
          defaultProps: {
            underline: "hover",
          },
        },
      },
    })
  );
}

export default theme;
