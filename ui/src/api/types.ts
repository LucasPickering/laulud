/*
 * This file holds all the types that our API accepts/returns. These types need
 * to exactly match what the API has. Good luck!
 */

export interface Image {
  url: string;
  width: number | null;
  height: number | null;
}

export interface CurrentUser {
  id: string;
  href: string;
  uri: string;
  display_name: string | null;
}

export interface Track {
  track: {
    id: string;
    name: string;
    href: string;
    uri: string;
    explicit: boolean;
    popularity: number;
    track_number: number;
  };
  tags: string[];
}
