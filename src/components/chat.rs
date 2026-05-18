use serde::{Deserialize, Serialize};
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew_agent::{Bridge, Bridged};

use crate::services::event_bus::EventBus;
use crate::{services::websocket::WebsocketService, User};

pub enum Msg {
    HandleMsg(String),
    SubmitMessage,
}

#[derive(Deserialize)]
struct MessageData {
    from: String,
    message: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum MsgTypes {
    Users,
    Register,
    Message,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct WebSocketMessage {
    message_type: MsgTypes,
    data_array: Option<Vec<String>>,
    data: Option<String>,
}

#[derive(Clone)]
struct UserProfile {
    name: String,
    avatar: String,
}

pub struct Chat {
    users: Vec<UserProfile>,
    chat_input: NodeRef,
    _producer: Box<dyn Bridge<EventBus>>,
    wss: WebsocketService,
    messages: Vec<MessageData>,
}
impl Component for Chat {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        let (user, _) = ctx
            .link()
            .context::<User>(Callback::noop())
            .expect("context to be set");
        let wss = WebsocketService::new();
        let username = user.username.borrow().clone();

        let message = WebSocketMessage {
            message_type: MsgTypes::Register,
            data: Some(username.to_string()),
            data_array: None,
        };

        if let Ok(_) = wss
            .tx
            .clone()
            .try_send(serde_json::to_string(&message).unwrap())
        {
            log::debug!("message sent successfully");
        }

        Self {
            users: vec![],
            messages: vec![],
            chat_input: NodeRef::default(),
            wss,
            _producer: EventBus::bridge(ctx.link().callback(Msg::HandleMsg)),
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::HandleMsg(s) => {
                let msg: WebSocketMessage = serde_json::from_str(&s).unwrap();
                match msg.message_type {
                    MsgTypes::Users => {
                        let users_from_message = msg.data_array.unwrap_or_default();
                        self.users = users_from_message
                            .iter()
                            .map(|u| UserProfile {
                                name: u.into(),
                                avatar: format!(
                                    "https://avatars.dicebear.com/api/adventurer-neutral/{}.svg",
                                    u
                                )
                                .into(),
                            })
                            .collect();
                        return true;
                    }
                    MsgTypes::Message => {
                        let message_data: MessageData =
                            serde_json::from_str(&msg.data.unwrap()).unwrap();
                        self.messages.push(message_data);
                        return true;
                    }
                    _ => {
                        return false;
                    }
                }
            }
            Msg::SubmitMessage => {
                let input = self.chat_input.cast::<HtmlInputElement>();
                if let Some(input) = input {
                    let message = WebSocketMessage {
                        message_type: MsgTypes::Message,
                        data: Some(input.value()),
                        data_array: None,
                    };
                    if let Err(e) = self
                        .wss
                        .tx
                        .clone()
                        .try_send(serde_json::to_string(&message).unwrap())
                    {
                        log::debug!("error sending to channel: {:?}", e);
                    }
                    input.set_value("");
                };
                false
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let submit = ctx.link().callback(|_| Msg::SubmitMessage);

        html! {
            <div class="flex w-screen h-screen bg-teal-50 font-sans">

                <div class="flex flex-col w-60 h-screen bg-white border-r border-teal-100 flex-shrink-0">

                    <div class="flex items-center gap-2 px-4 py-4 border-b border-teal-100">
                        <span class="text-base font-semibold text-teal-700">{"Yew Chat"}</span>
                    </div>

                    <div class="px-4 pt-4 pb-1">
                        <span class="text-xs font-semibold tracking-widest text-teal-400 uppercase">
                            {"Online"}
                        </span>
                    </div>

                    <div class="flex flex-col gap-1 px-3 overflow-y-auto flex-grow">
                        {
                            self.users.clone().iter().map(|u| {
                                html! {
                                    <div class="flex items-center gap-3 px-3 py-2 rounded-xl hover:bg-teal-50 transition-colors cursor-pointer">
                                        <div class="relative flex-shrink-0">
                                            <img
                                                class="w-9 h-9 rounded-full border-2 border-teal-100"
                                                src={u.avatar.clone()}
                                                alt="avatar"
                                            />
                                            <span class="absolute bottom-0 right-0 w-2.5 h-2.5 bg-green-400 border-2 border-white rounded-full"></span>
                                        </div>
                                        <div class="min-w-0">
                                            <p class="text-sm font-medium text-gray-800 truncate">{u.name.clone()}</p>
                                            <p class="text-xs text-teal-400 truncate">{"Hi there!"}</p>
                                        </div>
                                    </div>
                                }
                            }).collect::<Html>()
                        }
                    </div>


                </div>

                <div class="flex flex-col flex-grow h-screen min-w-0">

                    <div class="flex items-center justify-between px-6 h-14 bg-white border-b border-teal-100 flex-shrink-0">
                        <div class="flex items-center gap-2">
                            <span class="w-2 h-2 rounded-full bg-teal-400"></span>
                            <span class="text-base font-semibold text-gray-800">{"Group Chat"}</span>
                            <span class="text-xs text-teal-600 bg-teal-50 px-2 py-0.5 rounded-full font-medium">
                                {format!("{} online", self.users.len())}
                            </span>
                        </div>
                    </div>

                    <div class="flex flex-col flex-grow overflow-y-auto px-6 py-5 gap-4">
                        {
                            self.messages.iter().map(|m| {
                                let user = self.users.iter().find(|u| u.name == m.from);
                                let avatar = user.map(|u| u.avatar.clone()).unwrap_or_default();

                                html! {
                                    <div class="flex items-end gap-2">
                              
                                        <img
                                            class="w-7 h-7 rounded-full border border-teal-100 flex-shrink-0 mb-0.5"
                                            src={avatar}
                                            alt="avatar"
                                        />
                                 
                                        <div class="flex flex-col max-w-sm">
                                            <span class="text-xs text-teal-400 mb-1 pl-1 font-medium">
                                                {m.from.clone()}
                                            </span>
                                            <div class="bg-white border border-teal-100 rounded-2xl rounded-bl-sm px-4 py-2.5 shadow-sm">
                                                if m.message.ends_with(".gif") {
                                                    <img class="rounded-xl max-w-xs" src={m.message.clone()} />
                                                } else {
                                                    <p class="text-sm text-gray-700 leading-relaxed">
                                                        {m.message.clone()}
                                                    </p>
                                                }
                                            </div>
                                        </div>
                                    </div>
                                }
                            }).collect::<Html>()
                        }
                    </div>

          
                    <div class="px-4 py-3 bg-white border-t border-teal-100 flex-shrink-0">
                        <div class="flex items-center gap-2 bg-teal-50 border border-teal-100 rounded-2xl px-4 py-2">
                            <input
                                ref={self.chat_input.clone()}
                                type="text"
                                placeholder="Type a message…"
                                class="flex-grow bg-transparent text-sm text-gray-700 placeholder-teal-300 outline-none"
                                name="message"
                                required=true
                            />
                            <button
                                onclick={submit}
                                class="flex items-center justify-center w-8 h-8 rounded-xl bg-teal-500 hover:bg-teal-600 active:scale-95 transition-all flex-shrink-0"
                            >
                                <svg class="w-4 h-4 fill-white" viewBox="0 0 24 24">
                                    <path d="M0 0h24v24H0z" fill="none"/>
                                    <path d="M2.01 21L23 12 2.01 3 2 10l15 2-15 2z"/>
                                </svg>
                            </button>
                        </div>
                    </div>
                </div>
            </div>
        }
    }
}