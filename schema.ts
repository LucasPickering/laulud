
// https://developer.spotify.com/documentation/web-api/#spotify-uris-and-ids
export type SpotifyId = string;

export type SpotifyUri = string;

// All types that get serialized over the wire live here
// Any object type that can get a URI
export enum SpotifyObjectType { track = "track", album = "album", artist = "artist", user = "user" };

// https://developer.spotify.com/documentation/web-api/reference/object-model/#image-object
export type Image = { url: string; width: number | null; height: number | null };

// https://developer.spotify.com/documentation/web-api/reference/users-profile/get-current-users-profile/
export type CurrentUser = {     id: SpotifyId; href: string; uri: SpotifyUri; display_name: string     | null; images: Image [] };

// https://developer.spotify.com/documentation/web-api/reference/object-model/#external-id-object
export type ExternalIds = { [key: string]: string };

// https://developer.spotify.com/documentation/web-api/reference/object-model/#external-url-object
export type ExternalUrls = { [key: string]: string };

// https://developer.spotify.com/documentation/web-api/reference/object-model/#artist-object-simplified
export type ArtistSimplified = {     external_urls: ExternalUrls; href: string; id: SpotifyId; name:     string; uri: SpotifyUri };

// https://developer.spotify.com/documentation/web-api/reference/object-model/#artist-object-full
export type Artist = {     external_urls: ExternalUrls; genres: string []; href: string; id:     SpotifyId; images: Image []; name: string; popularity: number; uri: SpotifyUri };

// https://developer.spotify.com/documentation/web-api/reference/object-model/#album-object-simplified
export type AlbumSimplified = {     album_group: string | null; album_type: string; artists:     ArtistSimplified []; available_markets: string []; external_urls:     ExternalUrls; href: string; id: SpotifyId; images: Image []; name:     string; release_date: string; release_date_precision: string; uri:     SpotifyUri };

// https://developer.spotify.com/documentation/web-api/reference/object-model/#track-link
export type TrackLink = {     external_urls: ExternalUrls; href: string; id: SpotifyId; uri:     SpotifyUri };

// https://developer.spotify.com/documentation/web-api/reference/object-model/#track-object-full
export type Track = {     album: AlbumSimplified; artists: ArtistSimplified [];     available_markets: string []; disc_number: number; duration_ms:     number; explicit: boolean; external_ids: ExternalIds; external_urls:     ExternalUrls; href: string; id: SpotifyId; is_playable: boolean |     null; linked_from: TrackLink | null; name: string; popularity:     number; preview_url: string | null; track_number: number; uri:     SpotifyUri };

// Anything that can be tagged
// impl is in the util folder
export type Item = 
 | { type: "track"; data: Track } 
 | { type: "album"; data: AlbumSimplified } 
 | { type: "artist"; data: Artist };

// A taggable item, with its assigned tags
export type TaggedItem = { item: Item; tags: string [] };

// Response for a search query
export type ItemSearchResponse = { tracks: TaggedItem []; albums: TaggedItem []; artists: TaggedItem [] };

// Summary information for a tag
export type TagSummary = { tag: string; num_items: number };

// Details for a tag
export type TagDetails = { tag: string; items: TaggedItem [] };

// POST input for tagging a track
export type CreateTagBody = { tag: string };
