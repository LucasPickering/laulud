import React from "react";
import { CircularProgress } from "@material-ui/core";
import { useQuery } from "react-query";
import { useParams } from "react-router-dom";

import fd from "api/fd";
import { Track } from "api/types";

// Route params
interface Params {
  query: string;
}

const TrackSearchPage: React.FC = () => {
  const { query } = useParams<Params>();
  const { isLoading, data: tracks } = useQuery("track-search", () =>
    fd<Track[]>(`/api/tracks/search/${query}`)
  );

  if (isLoading) {
    return <CircularProgress />;
  }

  if (!tracks) {
    return <>shit</>;
  }

  return (
    <ul>
      {tracks.map((track) => (
        <li key={track.track_id}>{track.track_id}</li>
      ))}
    </ul>
  );
};

export default TrackSearchPage;
