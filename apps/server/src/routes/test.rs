use axum::response::sse::{Event, Sse};
use futures::stream::{self, Stream};
use std::{convert::Infallible, time::Duration};
use tokio_stream::StreamExt;

pub async fn mount() -> Sse<impl Stream<Item = Result<Event, Infallible>>> {
    let stream = stream::iter(1..=10)
        .map(|i| Event::default().data(i.to_string()))
        .throttle(Duration::from_secs(1));

    Sse::new(stream.map(Ok))
}
