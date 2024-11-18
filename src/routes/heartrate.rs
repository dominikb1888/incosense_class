use actix_web::{web::Bytes, Error, HttpResponse};
use chrono::{DateTime, Utc};
use futures_util::stream;
use rand::Rng;
use serde::{Deserialize, Serialize};
use serde_json;
use std::time::Duration;
use tokio::time::sleep;

#[derive(Serialize, Deserialize)]
struct Heartrate {
    voltage: f32,
    timestamp: DateTime<Utc>,
}

// https://www.hl7.org/fhir/observation-example-sample-data.json.html

pub async fn stream_heartrate() -> HttpResponse {
    let voltage_stream = stream::unfold(0, |state| async move {
        sleep(Duration::from_millis(100)).await;
        let mut rng = rand::thread_rng();
        // Simulate raw voltage data (e.g., between -1.0 and 1.0 volts)
        let voltage = rng.gen_range(-1.0..1.0);
        let heartrate = Heartrate {
            voltage,
            timestamp: Utc::now(),
        };
        let data = format!(
            "data: {}\n\n",
            serde_json::to_string(&heartrate).expect("Deserialization went wrong")
        );
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
