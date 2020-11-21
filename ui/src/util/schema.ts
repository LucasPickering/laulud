
// https://developer.spotify.com/documentation/web-api/reference/object-model/#paging-object
export type PaginatedResponse<T> = {     href: string; limit: number; offset: number; total: number; next:     string | null; previos: string | null; items: T [] };

// https://developer.spotify.com/documentation/web-api/reference/object-model/#image-object
export type Image = { url: string; width: number | null; height: number | null };

// https://developer.spotify.com/documentation/web-api/reference/users-profile/get-current-users-profile/
export type CurrentUser = {     id: string; href: string; uri: string; display_name: string | null; images: Image [] };

// https://developer.spotify.com/documentation/web-api/reference/object-model/#external-id-object
export type ExternalIds = { [key: string]: string };

// https://developer.spotify.com/documentation/web-api/reference/object-model/#external-url-object
export type ExternalUrls = { [key: string]: string };

// https://developer.spotify.com/documentation/web-api/reference/object-model/#artist-object-simplified
export type ArtistSimplified = {     external_urls: ExternalUrls; href: string; id: string; name: string; uri: string };

// https://developer.spotify.com/documentation/web-api/reference/object-model/#album-object-simplified
export type AlbumSimplified = {     album_group: string | null; album_type: string; artists:     ArtistSimplified []; available_markets: string []; external_urls:     ExternalUrls; href: string; id: string; images: Image []; name:     string; release_date: string; release_date_precision: string; uri:     string };

// https://developer.spotify.com/documentation/web-api/reference/object-model/#track-link
export type TrackLink = { external_urls: ExternalUrls; href: string; id: string; uri: string };

// https://developer.spotify.com/documentation/web-api/reference/object-model/#track-object-full
export type Track = {     album: AlbumSimplified; artists: ArtistSimplified [];     available_markets: string []; disc_number: number; duration_ms:     number; explicit: boolean; external_ids: ExternalIds; external_urls:     ExternalUrls; href: string; id: string; is_playable: boolean;     linked_from: TrackLink | null; name: string; popularity: number;     preview_url: string | null; track_number: number; uri: string };

// https://developer.spotify.com/documentation/web-api/reference/search/search/
export type TracksSearchResponse = { tracks: PaginatedResponse<Track>};

// A track that we've annotated with tag metadata
export type TaggedTrack = { track: Track; tags: string [] };

// POST input for tagging a trackS
export type CreateTagBody = { tags: string [] };
