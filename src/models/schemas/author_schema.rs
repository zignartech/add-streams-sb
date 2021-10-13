use serde::{Deserialize,Serialize};
use validator::Validate;
use iota_streams::ddml::types::NBytes;
use iota_streams::ddml::types::*;

pub use iota_streams::ddml::types::typenum;

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct AuthorSchema {
  pub seed: String,
  pub address: Address,
  pub author: Author,
}
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct Author {
  pub password: String,
  pub state: String,
  pub seed: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct Address {
  pub appInst: String,
  pub msgId: String,
  // pub msg_link: String
  // pub seq_num: u32,
}