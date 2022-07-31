use im::Vector;
use redux_rs::Store;
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, Debug)]
pub enum ChatMessage {
    System(String),
    Message(String, String),
    SecretMessage(Vec<String>, String),
}

#[derive(Default, Debug)]
pub struct Chat {
    pub messages: Vector<ChatMessage>,
}

impl Chat {
    pub fn get_my_messages(&self, user: &str) -> Vec<ChatMessage> {
        let user = user.to_owned();
        self.messages
            .iter()
            .filter_map(|msg| -> Option<ChatMessage> {
                match msg {
                    ChatMessage::SecretMessage(targets, _) => {
                        if targets.contains(&user) {
                            Some(msg.to_owned())
                        } else {
                            None
                        }
                    }
                    _ => Some(msg.to_owned()),
                }
            })
            .collect::<Vec<ChatMessage>>()
    }
}

pub enum ChatAction {
    PostMessage(ChatMessage),
}

fn reducer(state: Chat, action: ChatAction) -> Chat {
    match action {
        ChatAction::PostMessage(message) => Chat {
            messages: {
                let mut messages = state.messages;
                messages.push_back(message);
                messages
            },
        },
    }
}

pub fn create_chat_store() -> Store<Chat, ChatAction, fn(Chat, ChatAction) -> Chat> {
    Store::new(reducer)
}

#[cfg(test)]
mod test {
    use im::vector;

    use super::*;

    #[test]
    fn new_chat_is_empty() {
        let chat = Chat::default();
        assert_eq!(chat.messages.len(), 0);
    }

    #[test]
    fn add_message_to_empty_chat() {
        let chat = Chat::default();
        let chat = reducer(
            chat,
            ChatAction::PostMessage(ChatMessage::System("System Message".to_owned())),
        );
        assert_eq!(chat.messages.len(), 1);
        if let ChatMessage::System(msg) = &chat.messages[0] {
            assert_eq!(msg, "System Message");
        } else {
            panic!()
        }
    }

    #[test]
    fn add_message_to_existing_chat() {
        let original = Chat {
            messages: vector![ChatMessage::System("A Message".to_owned())],
        };
        let chat = reducer(
            original,
            ChatAction::PostMessage(ChatMessage::System("System Message".to_owned())),
        );
        assert_eq!(chat.messages.len(), 2);
        if let ChatMessage::System(msg) = &chat.messages[1] {
            assert_eq!(msg, "System Message");
        } else {
            panic!()
        }
    }

    #[test]
    fn filters_out_secret_messages_for_others() {
        let chat = Chat {
            messages: vector![
                ChatMessage::SecretMessage(vec!["you".to_owned()], "Test 1".to_owned()),
                ChatMessage::System("A Message".to_owned()),
                ChatMessage::SecretMessage(vec!["them".to_owned()], "Test 2".to_owned())
            ],
        };

        let result = chat.get_my_messages("me");
        assert_eq!(result.len(), 1);
        if let ChatMessage::System(msg) = &result[0] {
            assert_eq!(msg, "A Message");
        } else {
            panic!()
        }
    }

    #[test]
    fn leaves_in_secret_messages_to_me() {
        let chat = Chat {
            messages: vector![
                ChatMessage::SecretMessage(vec!["you".to_owned()], "Test 1".to_owned()),
                ChatMessage::System("A Message".to_owned()),
                ChatMessage::SecretMessage(vec!["them".to_owned()], "Test 2".to_owned())
            ],
        };

        let result = chat.get_my_messages("you");
        assert_eq!(result.len(), 2);
        if let ChatMessage::SecretMessage(_, msg) = &result[0] {
            assert_eq!(msg, "Test 1");
        } else {
            panic!()
        }
        if let ChatMessage::System(msg) = &result[1] {
            assert_eq!(msg, "A Message");
        } else {
            panic!()
        }
    }
}
