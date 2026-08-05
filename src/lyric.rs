use reqwest::Method;
use reqwest::blocking::Client;
use serde::{Deserialize, Serialize};

use crate::API_URL;
use crate::error::LrcLibError;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Lyric {
    pub id: i64,
    pub name: String,
    pub track_name: String,
    pub artist_name: String,
    pub album_name: String,
    pub duration: f64,
    pub instrumental: bool,
    pub plain_lyrics: String,
    pub synced_lyrics: String,
    #[serde(alias = "lyricsfile")]
    pub lyrics_file: String,
}

impl Lyric {
    pub fn by_song(
        track_name: String,
        artist: String,
        album: Option<String>,
        duration: Option<String>,
    ) -> Result<Lyric, LrcLibError> {
        let client = Client::new();
        let mut url = format!("{API_URL}/get?track_name={track_name}&artist_name={artist}");
        if let Some(album) = album {
            url += &album;
        }
        if let Some(duration) = duration {
            url += &duration;
        }
        let res = client.request(Method::GET, url).send()?;
        if res.status().as_u16() == 404 {
            return Err(LrcLibError::TrackNotFound {
                name: track_name.into(),
            });
        }
        serde_json::from_reader::<_, Lyric>(res).map_err(|err| err.into())
    }
}
