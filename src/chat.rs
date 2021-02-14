use actix::{Actor, StreamHandler};
use actix_web::{web, Error, HttpRequest, HttpResponse};
use actix_web_actors::ws;

/// Define HTTP actor
struct MyWs;

impl Actor for MyWs {
  type Context = ws::WebsocketContext<Self>;
}

/// Handler for ws::Message message
impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for MyWs {
  fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
    match msg {
      Ok(ws::Message::Ping(msg)) => ctx.pong(&msg),
      Ok(ws::Message::Text(text)) => {
        use rust_bert::pipelines::conversation::{ConversationManager, ConversationModel};
        let conversation_model = ConversationModel::new(Default::default()).unwrap();
        let mut conversation_manager = ConversationManager::new();

        let conversation_id = conversation_manager.create(text.as_str());
        ctx.text(
          *conversation_model
            .generate_responses(&mut conversation_manager)
            .get(&conversation_id)
            .unwrap(),
        )
      }
      Ok(ws::Message::Binary(bin)) => ctx.binary(bin),
      _ => (),
    }
  }
}

pub async fn index(req: HttpRequest, stream: web::Payload) -> Result<HttpResponse, Error> {
  let resp = ws::start(MyWs {}, &req, stream);
  println!("{:?}", resp);
  resp
}
