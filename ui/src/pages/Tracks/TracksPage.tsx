import React, { useState } from "react";
import { Grid } from "@material-ui/core";

import { TaggedTrack } from "util/schema";
import TrackSearchList from "./TrackSearchList";
import TrackDetails from "./TrackDetails";

const TracksPage: React.FC = () => {
  const [selectedTrack, setSelectedTrack] = useState<TaggedTrack | undefined>();

  return (
    <Grid container spacing={2}>
      <Grid item xs={12} sm={6} md={4}>
        <TrackSearchList
          selectedTrack={selectedTrack}
          setSelectedTrack={setSelectedTrack}
        />
      </Grid>
      {selectedTrack && (
        <Grid item xs={12} sm={6} md={8}>
          <TrackDetails track={selectedTrack} />
        </Grid>
      )}
    </Grid>
  );
};

export default TracksPage;
