/**
 * Hand-written mappings of the Rust API's types. This file defines strict types
 * related to the API, and must be kept up to date with the Rust code.
 *
 * Obviously this is less than ideal, we can replace with this an OpenAPI
 * mapping in https://github.com/LucasPickering/laulud/issues/8, once Rocket
 * v0.5 is stabilized.
 */

import {
  CurrentUser,
  ItemSearchResponse,
  SpotifyUri,
  TagDetails,
  TaggedItem,
  TagSummary,
} from "schema";

export type ApiRouteCurrentUser = ["users", "current"];
export type ApiRouteItem = ["items", SpotifyUri];
export type ApiRouteItemSearch = ["items", "search", string];
export type ApiRouteTags = ["tags"];
export type ApiRouteTag = ["tags", string];
/**
 * All possible API GET routes. Used for type-enforcing query operations.
 */
export type ApiRoute =
  | ApiRouteCurrentUser
  | ApiRouteItem
  | ApiRouteItemSearch
  | ApiRouteTags
  | ApiRouteTag;

/**
 * Map API route to the route's return type. This MUST be kept in sync with the
 * Rust code, otherwise we'll have problemos in TS. This only maps GET routes,
 * not POST/DELETE/etc.
 */
// prettier-ignore
export type ApiOutputType<T extends ApiRoute> =
  // /api/users/current
  T extends ApiRouteCurrentUser ? CurrentUser
  // /api/items/<uri>
  : T extends ApiRouteItem ? TaggedItem
  // /api/items/search/<query>
  : T extends ApiRouteItemSearch ? ItemSearchResponse
  // /api/tags
  : T extends ApiRouteTags ? TagSummary[]
  // /api/tags/<tag>
  : T extends ApiRouteTag ? TagDetails
  // Unmapped routes (should be none)
  : never;
