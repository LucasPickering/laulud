import React, { useState } from "react";
import { useParams } from "react-router-dom";
import { Grid } from "@material-ui/core";

import TrackSearchList from "./TrackSearchList";
import TrackDetails from "./TrackDetails";

interface RouteParams {
  trackId?: string;
}

const TracksPage: React.FC = () => {
  const { trackId } = useParams<RouteParams>();
  const [query, setQuery] = useState<string>("");

  return (
    <Grid container spacing={2}>
      <Grid item xs={12} sm={6} md={4}>
        <TrackSearchList
          query={query}
          setQuery={setQuery}
          selectedTrackId={trackId}
        />
      </Grid>
      {trackId && (
        <Grid item xs={12} sm={6} md={8}>
          <TrackDetails
            trackId={trackId}
            trackListQueryKey={["tracks", { query }]}
          />
        </Grid>
      )}
    </Grid>
  );
};

export default TracksPage;