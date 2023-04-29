use std::collections::{HashMap, HashSet};

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
    NeighbourBroadcast(BroadcastMessageBody),
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
    messages: HashSet<i32>,
    msg_id: u32,
    neighbours: Vec<String>,
    name: Option<String>,
}

impl Node {
    fn broadcast(&mut self, message: i32) -> anyhow::Result<()> {
        if self.messages.insert(message) {
            for neighbour in self.neighbours.clone() {
                self.send_message(
                    neighbour,
                    None,
                    MessageType::NeighbourBroadcast(BroadcastMessageBody { message }),
                )?;
            }
        }

        Ok(())
    }
}

impl MaelstromNodoe<MessageType> for Node {
    fn create_reply_message_type(
        &mut self,
        message_type: MessageType,
    ) -> anyhow::Result<Option<MessageType>> {
        match message_type {
            MessageType::Broadcast(broadcast) => {
                self.broadcast(broadcast.message)?;

                Ok(Some(MessageType::BroadcastOk))
            }
            MessageType::BroadcastOk => panic!("Should not receive broadcast_ok message"),
            MessageType::Init(init) => {
                self.name = Some(init.node_id);

                Ok(Some(MessageType::InitOk))
            }
            MessageType::InitOk => panic!("Should not receive init_ok message"),
            MessageType::NeighbourBroadcast(broadcast) => {
                self.broadcast(broadcast.message)?;

                Ok(None)
            }
            MessageType::Read => Ok(Some(MessageType::ReadOk(ReadOkMessageBody {
                messages: self.messages.clone(),
            }))),
            MessageType::ReadOk(_) => panic!("Should not receive read_ok message"),
            MessageType::Topology(mut topology) => {
                self.neighbours = topology
                    .topology
                    .remove(self.name.as_ref().unwrap())
                    .unwrap();

                Ok(Some(MessageType::TopologyOk))
            }
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
    messages: HashSet<i32>,
}

#[derive(Deserialize, Serialize)]
struct TopologyMessageBody {
    topology: HashMap<String, Vec<String>>,
}

fn main() -> anyhow::Result<()> {
    let mut node = Node {
        messages: HashSet::new(),
        msg_id: 0,
        neighbours: Vec::new(),
        name: None,
    };
    node.start()?;

    Ok(())
}
