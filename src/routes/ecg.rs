use actix_web::{web::Bytes, Error, HttpResponse};
use futures_util::stream;
use rand::Rng;
use std::time::Duration;
use tokio::time::sleep;

pub async fn stream_voltage() -> HttpResponse {
    let voltage_stream = stream::unfold(0, |state| async move {
        sleep(Duration::from_millis(100)).await;
        let mut rng = rand::thread_rng();
        // Simulate raw voltage data (e.g., between -1.0 and 1.0 volts)
        let voltage = rng.gen_range(-1.5..1.5);
        let data = format!("data: {}\n\n", voltage);
        Some((
            Ok::<actix_web::web::Bytes, Error>(Bytes::from(data)),
            state + 1,
        ))
    });

    HttpResponse::Ok()
        .content_type("text/event-stream")
        .insert_header(("Cache-Control", "no-cache"))
        .streaming(voltage_stream)
}
