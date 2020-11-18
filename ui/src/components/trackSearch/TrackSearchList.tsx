import React from "react";
import { CircularProgress } from "@material-ui/core";
import { useQuery } from "react-query";

import fd from "api/fd";
import { Track } from "api/types";

interface Props {
  query: string;
}

const TrackSearchList: React.FC<Props> = ({ query }) => {
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

export default TrackSearchList;
