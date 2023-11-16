#![cfg(feature = "ssr")]
use actix::*;
use actix_broker::BrokerSubscribe;
use actix_web::{web, Error, HttpRequest, HttpResponse};
use actix_web_actors::ws;
use serde::{Deserialize, Serialize};

#[derive(Message, Clone, Serialize, Deserialize)]
#[rtype(result = "()")]
pub struct RemSeatsMsg(pub Vec<(super::SubjectId, u32)>);

struct RemSeatsWs {
    db_pool: sqlx::SqlitePool,
}

impl RemSeatsWs {
    pub fn new(db_pool: sqlx::SqlitePool) -> Self {
        Self { db_pool }
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
            super::server_fns::get_rem_seats(&[], pool)
                .await
                .unwrap_or(RemSeatsMsg(Vec::new()))
        }));
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
            Ok(ws::Message::Ping(msg)) => ctx.pong(&msg),
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
    pool: web::Data<sqlx::SqlitePool>,
    stream: web::Payload,
) -> Result<HttpResponse, Error> {
    // TODO: check for auth
    // TODO: check if registration is active for student_id
    ws::start(RemSeatsWs::new(pool.get_ref().clone()), &req, stream)
}
