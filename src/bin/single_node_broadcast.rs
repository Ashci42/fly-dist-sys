use std::collections::HashMap;

use maelstrom::{InitMessageBody, MaelstromNodoe};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
enum MessageType {
    Broadcast(BroadcastMessageBody),
    BroadcastOk,
    Init(InitMessageBody),
    InitOk,
    Read,
    ReadOk(ReadOkMessageBody),
    Topology(TopologyMessageBody),
    TopologyOk,
}

#[derive(Deserialize, Serialize)]
struct BroadcastMessageBody {
    message: i32,
}

struct Node {
    messages: Vec<i32>,
    msg_id: u32,
    name: Option<String>,
}

impl MaelstromNodoe<MessageType> for Node {
    fn create_reply_message_type(&mut self, message_type: MessageType) -> MessageType {
        match message_type {
            MessageType::Broadcast(broadcast) => {
                self.messages.push(broadcast.message);

                MessageType::BroadcastOk
            }
            MessageType::BroadcastOk => panic!("Should not receive broadcast_ok message"),
            MessageType::Init(init) => {
                self.name = Some(init.node_id);

                MessageType::InitOk
            }
            MessageType::InitOk => panic!("Should not receive init_ok message"),
            MessageType::Read => MessageType::ReadOk(ReadOkMessageBody {
                messages: self.messages.clone(),
            }),
            MessageType::ReadOk(_) => panic!("Should not receive read_ok message"),
            MessageType::Topology(_) => MessageType::TopologyOk,
            MessageType::TopologyOk => todo!(),
        }
    }

    fn get_next_msg_id(&mut self) -> u32 {
        self.msg_id += 1;

        self.msg_id
    }

    fn get_node_name(&self) -> &Option<String> {
        &self.name
    }
}

#[derive(Deserialize, Serialize)]
struct ReadOkMessageBody {
    messages: Vec<i32>,
}

#[derive(Deserialize, Serialize)]
struct TopologyMessageBody {
    topology: HashMap<String, Vec<String>>,
}

fn main() -> anyhow::Result<()> {
    let mut node = Node {
        messages: Vec::new(),
        msg_id: 0,
        name: None,
    };
    node.start()?;

    Ok(())
}
