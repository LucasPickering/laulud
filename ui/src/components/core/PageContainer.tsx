import React, { Suspense } from "react";
import { Box, Divider, Stack, Typography } from "@mui/material";
import Loading from "components/Loading";
import Link from "../generic/Link";
import Header from "./Header";

interface Props {
  showHeader?: boolean;
}

/**
 * Container for all content on the page. This is used in the root to wrap all
 * pages.
 */
const PageContainer: React.FC<Props> = ({ showHeader = true, children }) => (
  <Box display="flex" flexDirection="column" alignItems="center" height="100%">
    {showHeader && <Header />}

    <Box width="100%" maxWidth={1280} padding={2} paddingBottom={0}>
      <Suspense fallback={<Loading />}>{children}</Suspense>
    </Box>

    <Stack
      component="footer"
      marginTop="auto"
      direction="row"
      divider={<Divider orientation="vertical" />}
      padding={2}
      spacing={1}
    >
      <Typography variant="body2">
        Created by <Link to="https://lucaspickering.me">Lucas Pickering</Link>
      </Typography>
      <Link variant="body2" to="https://github.com/LucasPickering/laulud">
        GitHub
      </Link>
    </Stack>
  </Box>
);

export default PageContainer;
