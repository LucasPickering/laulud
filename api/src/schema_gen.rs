use crate::{schema::*, LauludConfig};
use log::info;
use std::{
    borrow::Cow,
    fs::File,
    io::{self, Write},
};
use typescript_definitions::TypeScriptifyTrait;

/// All types that get serialized over the wire live here

const TS_DEFINITION_GENERATION_FUNCS: &[&dyn Fn() -> Cow<'static, str>] = &[
    // Hack for generic structs, the type it generates is still generic
    &<PaginatedResponse<i32>>::type_script_ify,
    &Image::type_script_ify,
    &CurrentUser::type_script_ify,
    &ExternalIds::type_script_ify,
    &ExternalUrls::type_script_ify,
    &ArtistSimplified::type_script_ify,
    &AlbumSimplified::type_script_ify,
    &TrackLink::type_script_ify,
    &Track::type_script_ify,
    &TracksResponse::type_script_ify,
    &TracksSearchResponse::type_script_ify,
    &TaggedTrack::type_script_ify,
    &TagSummary::type_script_ify,
    &TagDetails::type_script_ify,
    &CreateTagBody::type_script_ify,
    // Make sure any new types get added here
];

pub fn generate_ts_definitions(config: &LauludConfig) -> io::Result<()> {
    if let Some(path) = &config.ts_definitions_file {
        let mut file =
            File::with_options().create(true).write(true).open(path)?;

        for func in TS_DEFINITION_GENERATION_FUNCS {
            file.write_all(b"\n")?;
            file.write_all(func().as_bytes())?;
            file.write_all(b"\n")?;
        }

        file.sync_all()?;
        info!(
            "Generated TypeScript definitions at {}",
            path.to_str().unwrap()
        );
    }
    Ok(())
}
