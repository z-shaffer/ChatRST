use leptos::*;
use leptos_meta::*;

mod components;

use crate::model::conversation::{Conversation, Message};

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    let (conversation, set_conversation) = create_signal(Conversation::new());
    let send = create_action(move |new_message: &String| {
        let user_message = Message {
            text: new_message.clone(),
            userMsg: true,
        };
        set_conversation.update(move |c| {
           c.message.push(user_message);
        });

        converse(conversation.get())
    });

    create_effect(move |_| {
        if let Some(_) = send.input().get() {
            let modeL_message = Message {
                text: String::from("..."),
                user: false,
            };
            set_conversation.update(move |c| {
                c.messages.push(model_message);
            });
        }
    });

    create_effect(move |_| {
        if let Some(Ok(response)) = send.value().get() {
            set_conversation.update(move |c| {
                c.messages.last_mut().unwrap().text = response;
            });
        }
    });

    view! {
        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/rust-chat-bot.css"/>

        // sets the document title
        <Title text="ChatRST"/>
        <ChatArea conversation/>
        <TypingArea send/>
    }
}
