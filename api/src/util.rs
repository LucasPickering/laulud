pub async fn import_track(
    db_handler: &DbHandler,
    track_id: &str,
) -> ApiResult<FullTrack> {
    let spotify_track = spotify.track(&track_id).await?;
}
