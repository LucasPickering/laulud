import SimpleTable from "components/generic/SimpleTable";
import React from "react";
import { useFragment } from "react-relay";
import { graphql } from "relay-runtime";
import { TrackDetails_track$key } from "./__generated__/TrackDetails_track.graphql";

/**
 * Detailed information specific to a track
 */
const TrackDetails: React.FC<{
  trackKey: TrackDetails_track$key;
}> = ({ trackKey }) => {
  // For details on all these fields:
  // https://developer.spotify.com/documentation/web-api/reference/#object-audiofeaturesobject

  const { audioFeatures: features } = useFragment(
    graphql`
      fragment TrackDetails_track on Track {
        audioFeatures {
          acousticness
          danceability
          durationMs
          energy
          instrumentalness
          key
          liveness
          loudness
          mode
          speechiness
          tempo
          timeSignature
          valence
        }
      }
    `,
    trackKey
  );

  return (
    <SimpleTable
      data={[
        { label: "Duration", value: formatTime(features.durationMs) },
        { label: "Key", value: formatKey(features.key) },
        { label: "Mode", value: formatMode(features.mode) },
        { label: "Time Signature", value: features.timeSignature },
        { label: "Tempo", value: `${formatNum(features.tempo, 0)} bpm` },
        { label: "Loudness", value: `${formatNum(features.loudness, 1)} dB` },
        { label: "Acousticness", value: formatNum(features.acousticness) },
        { label: "Danceability", value: formatNum(features.danceability) },
        { label: "Energy", value: formatNum(features.energy) },
        {
          label: "Instrumentalness",
          value: formatNum(features.instrumentalness),
        },
        { label: "Liveness", value: formatNum(features.liveness) },
        { label: "Speechiness", value: formatNum(features.speechiness) },
        { label: "Valence", value: formatNum(features.valence) },
      ]}
    />
  );
};

function formatNum(value: number, places: number = 3): string {
  return value.toFixed(places);
}

/**
 * Format a millisecond quantity into an HH:MM:ss string
 */
function formatTime(ms: number): string {
  // Slice off just the timestamp portion (excluding fractional seconds)
  return new Date(ms).toISOString().slice(11, -5);
}

/**
 * Format a pitch class ID into a pretty string
 * https://en.wikipedia.org/wiki/Pitch_class#Other_ways_to_label_pitch_classes
 */
function formatKey(key: number): string {
  switch (key) {
    case 0:
      return "C";
    case 1:
      return "C♯/D♭";
    case 2:
      return "D";
    case 3:
      return "D♯/E♭";
    case 4:
      return "E";
    case 5:
      return "F";
    case 6:
      return "F♯/G♭";
    case 7:
      return "G";
    case 8:
      return "G♯/A♭";
    case 9:
      return "A";
    case 10:
      return "A♯/B♭";
    case 11:
      return "B";
    default:
      throw new Error(`Unknown pitch class: ${key}`);
  }
}

/**
 * Format a mode ID (major/minor) into a string
 */
function formatMode(mode: number): string {
  switch (mode) {
    case 0:
      return "Minor";
    case 1:
      return "Major";
    default:
      throw new Error(`Unknown mode ID: ${mode}`);
  }
}

export default TrackDetails;
