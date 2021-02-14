use actix::{Actor, StreamHandler};
use actix_web::{web, Error, HttpRequest, HttpResponse};
use actix_web_actors::ws;
use rust_bert::pipelines::translation::Language as PrimitiveLangPair;
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Language {
  Catalan,
  English,
  French,
  German,
  Italian,
  Portuguese,
  Russian,
  Spanish,
}

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
/// A language pair used for translating different languages.
pub struct LanguagePair(Language, Language);

impl LanguagePair {
  const fn as_primitive_lang_pair(self) -> Option<PrimitiveLangPair> {
    use Language::*;
    use PrimitiveLangPair::*;
    let Self(l1, l2) = self;
    match l1 {
      English => match l2 {
        French => Some(EnglishToFrench),
        Spanish => Some(EnglishToSpanish),
        Portuguese => Some(EnglishToPortuguese),
        Italian => Some(EnglishToItalian),
        Catalan => Some(EnglishToCatalan),
        German => Some(EnglishToGerman),
        Russian => Some(EnglishToRussian),
        _ => None,
      },
      French => match l2 {
        English => Some(FrenchToEnglish),
        German => Some(FrenchToGerman),
        _ => None,
      },
      Spanish => match l2 {
        English => Some(SpanishToEnglish),
        _ => None,
      },
      Portuguese => match l2 {
        English => Some(PortugueseToEnglish),
        _ => None,
      },
      Italian => match l2 {
        English => Some(ItalianToEnglish),
        _ => None,
      },
      Catalan => match l2 {
        English => Some(CatalanToEnglish),
        _ => None,
      },
      German => match l2 {
        English => Some(GermanToEnglish),
        French => Some(GermanToFrench),
        _ => None,
      },
      Russian => match l2 {
        English => Some(RussianToEnglish),
        _ => None,
      },
    }
  }
}

use rust_bert::pipelines::translation::TranslationModel;

/// Define HTTP actor
struct TranslationWS {}

impl Actor for TranslationWS {
  type Context = ws::WebsocketContext<Self>;
}

#[derive(Serialize, Deserialize, Debug)]
struct TranslateInputRequest {
  pair: LanguagePair,
  input: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct TranslateInputResponse {
  output: String,
}

/// Handler for ws::Message message
impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for TranslationWS {
  fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
    match msg {
      Ok(ws::Message::Ping(msg)) => ctx.pong(&msg),
      Ok(ws::Message::Text(text)) => {
        //
        match serde_json::from_str::<TranslateInputRequest>(text.as_str()) {
          Ok(TranslateInputRequest { pair, input }) => {
            use rust_bert::pipelines::translation::TranslationConfig;
            use tch::Device;
            let translation_config = TranslationConfig::new(
              pair.as_primitive_lang_pair().unwrap(),
              Device::cuda_if_available(),
            );
            let model = TranslationModel::new(translation_config).unwrap();
            ctx.text(model.translate(&[input.as_str()])[0].as_str())
          }
          _ => ctx.text(format!("Unable to translate text.",)),
        }
      }
      Ok(ws::Message::Binary(bin)) => ctx.binary(bin),
      _ => (),
    }
  }
}

pub async fn index(req: HttpRequest, stream: web::Payload) -> Result<HttpResponse, Error> {
  ws::start(TranslationWS {}, &req, stream)
}
