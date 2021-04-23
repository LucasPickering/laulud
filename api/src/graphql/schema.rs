//! Schema generation for the API. This is in a separate module just to make
//! it easier to use `cargo expand` on just this macro when debugging stuff.

use super::*;
use crate::spotify::{
    AlbumSimplified, Artist, ArtistSimplified, Image, PrivateUser, Track,
};
use juniper_from_schema::graphql_schema_from_file;

graphql_schema_from_file!(
    "schema/*.graphql",
    context_type: RequestContext,
    error_type: ApiError
);
