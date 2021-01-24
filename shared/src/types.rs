use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, PartialEq, Debug)]
enum Message {
    EvalCommit(String),
}
