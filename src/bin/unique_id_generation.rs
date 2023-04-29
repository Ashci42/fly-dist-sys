use maelstrom::{InitMessageBody, MaelstromNodoe};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
enum MessageType {
    Generate,
    GenerateOk(GenerateOkMessageBody),
    Init(InitMessageBody),
    InitOk,
}

#[derive(Deserialize, Serialize)]
struct GenerateOkMessageBody {
    id: String,
}

struct Node {
    msg_id: u32,
    name: Option<String>,
}

impl MaelstromNodoe<MessageType> for Node {
    fn create_reply_message_type(
        &mut self,
        message_type: MessageType,
    ) -> anyhow::Result<Option<MessageType>> {
        match message_type {
            MessageType::Generate => Ok(Some(MessageType::GenerateOk(GenerateOkMessageBody {
                id: format!("{}-{}", self.name.as_ref().unwrap(), self.msg_id),
            }))),
            MessageType::GenerateOk(_) => panic!("Should not receive generate_ok message"),
            MessageType::Init(init) => {
                self.name = Some(init.node_id);

                Ok(Some(MessageType::InitOk))
            }
            MessageType::InitOk => panic!("Should not receive init_ok message"),
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

fn main() -> anyhow::Result<()> {
    let mut node = Node {
        msg_id: 0,
        name: None,
    };
    node.start()?;

    Ok(())
}
