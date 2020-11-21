/*
 * This file holds all the types that our API accepts/returns. These types need
 * to exactly match what the API has. Good luck!
 */

export interface CurrentUser {
  id: string;
  href: string;
  uri: string;
  display_name: string | null;
}

export interface Track {
  track_id: string;
  tags: string[];
}
