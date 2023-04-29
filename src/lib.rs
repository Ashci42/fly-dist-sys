use std::io::{self, BufRead, Write};

use serde::{de::DeserializeOwned, Deserialize, Serialize};

pub trait MaelstromNodoe<T>
where
    T: DeserializeOwned + Serialize,
{
    fn create_reply_message_type(&mut self, message_type: T) -> anyhow::Result<Option<T>>;
    fn get_next_msg_id(&mut self) -> u32;
    fn get_node_name(&self) -> &Option<String>;

    fn send_message(
        &mut self,
        dest: String,
        in_reply_to: Option<u32>,
        message_type: T,
    ) -> anyhow::Result<()> {
        let message = Message {
            body: MessageBody {
                in_reply_to,
                msg_id: Some(self.get_next_msg_id()),
                t: message_type,
            },
            dest,
            src: self.get_node_name().as_ref().unwrap().to_owned(),
        };
        let buf = serde_json::to_string(&message)?;
        io::stdout().write_all(buf.as_bytes())?;
        io::stdout().write_all(b"\n")?;

        Ok(())
    }

    fn start(&mut self) -> anyhow::Result<()> {
        let lines = io::stdin().lock().lines();
        for line in lines {
            let line = line?;
            let message: Message<T> = serde_json::from_str(&line)?;
            let message_type_response = self.create_reply_message_type(message.body.t)?;
            if let Some(message_type_response) = message_type_response {
                self.send_message(message.src, message.body.msg_id, message_type_response)?;
            }
        }

        Ok(())
    }

    fn log() {
        todo!();
    }
}

#[derive(Deserialize, Serialize)]
pub struct InitMessageBody {
    pub node_id: String,
    node_ids: Vec<String>,
}

#[derive(Deserialize, Serialize)]
struct Message<T> {
    body: MessageBody<T>,
    dest: String,
    src: String,
}

#[derive(Deserialize, Serialize)]
struct MessageBody<T> {
    in_reply_to: Option<u32>,
    msg_id: Option<u32>,
    #[serde(flatten)]
    t: T,
}
