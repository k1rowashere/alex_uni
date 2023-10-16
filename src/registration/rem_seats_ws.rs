#![cfg(feature = "ssr")]
use actix::*;
use actix_broker::BrokerSubscribe;
use actix_web::{web, Error, HttpRequest, HttpResponse};
use actix_web_actors::ws;
use serde::{Deserialize, Serialize};
use std::time::{Duration, Instant};

const HEARTBEAT_INTERVAL: Duration = Duration::from_secs(5);
const CLIENT_TIMEOUT: Duration = Duration::from_secs(10);

#[derive(Message, Clone, Serialize, Deserialize)]
#[rtype(result = "()")]
pub struct RemSeatsMsg(pub Vec<(super::SubjectId, u32)>);

struct RemSeatsWs {
    hb: Instant,
    db_pool: sqlx::SqlitePool,
}

impl RemSeatsWs {
    pub fn new(db_pool: sqlx::SqlitePool) -> Self {
        Self {
            hb: Instant::now(),
            db_pool,
        }
    }
}

impl Actor for RemSeatsWs {
    type Context = ws::WebsocketContext<Self>;
    fn started(&mut self, ctx: &mut Self::Context) {
        // Subscribe to the broker channel
        self.subscribe_system_async::<RemSeatsMsg>(ctx);

        let pool = self.db_pool.clone();
        // sends rem_seats to client on startup
        ctx.add_message_stream(futures::stream::once(async move {
            super::server_fns::get_rem_seats(None, pool)
                .await
                .unwrap_or(RemSeatsMsg(Vec::new()))
        }));

        // heartbeat: ping the client every 5 secs to check if the conn is alive
        // TEMP: is this needed?
        ctx.run_interval(HEARTBEAT_INTERVAL, |act, ctx| {
            if Instant::now().duration_since(act.hb) > CLIENT_TIMEOUT {
                ctx.stop();
            } else {
                ctx.ping(b"");
            }
        });
    }
}

impl Handler<RemSeatsMsg> for RemSeatsWs {
    type Result = ();
    fn handle(
        &mut self,
        msg: RemSeatsMsg,
        ctx: &mut Self::Context,
    ) -> Self::Result {
        ctx.text(
            serde_json::to_string(&msg.0).expect("This should never fail"),
        );
    }
}

/// Handler for `ws::Message` message
impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for RemSeatsWs {
    fn handle(
        &mut self,
        msg: Result<ws::Message, ws::ProtocolError>,
        ctx: &mut Self::Context,
    ) {
        match msg {
            Ok(ws::Message::Ping(msg)) => {
                self.hb = Instant::now();
                ctx.pong(&msg);
            }
            Ok(ws::Message::Pong(_)) => self.hb = Instant::now(),
            Ok(ws::Message::Close(reason)) => {
                ctx.close(reason);
                ctx.stop()
            }
            _ => (),
        }
    }
}

pub async fn rem_seats_ws(
    req: HttpRequest,
    stream: web::Payload,
) -> Result<HttpResponse, Error> {
    // TODO: check for auth
    // TODO: check if registration is active for student_id
    let pool = req
        .app_data::<sqlx::SqlitePool>()
        .expect("Expected DB pool in app data");

    ws::start(RemSeatsWs::new(pool.clone()), &req, stream)
}
