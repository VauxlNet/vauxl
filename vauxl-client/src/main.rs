#![allow(non_snake_case)] // Allow component names like App, ServerSidebar etc.

use dioxus::prelude::*;
use std::collections::HashMap;

// --- Data Structures ---
#[derive(Clone, PartialEq, Debug)]
struct Server {
    id: String,
    name: String,
    icon: String, // Could be URL or initial
    channels: Vec<Channel>,
}

#[derive(Clone, PartialEq, Debug)]
struct Channel {
    id: String,
    name: String,
    channel_type: ChannelType,
    parent_id: Option<String>, // Added for categorization
    messages: Vec<Message>,
}

#[derive(Clone, PartialEq, Debug)]
enum ChannelType {
    Text,
    Voice,
    Category,
}

#[derive(Clone, PartialEq, Debug)]
struct Message {
    id: String,
    content: String,
    author: User,
    timestamp: String, // Consider using a proper DateTime type
}

#[derive(Clone, PartialEq, Debug)]
struct User {
    id: String,
    username: String,
    avatar: String, // URL
    status: UserStatus,
}

#[derive(Clone, PartialEq, Debug)]
enum UserStatus {
    Online,
    Idle,
    DoNotDisturb,
    Offline,
}

// --- Main Entry Point ---
fn main() {
    // Platform specific launchers
    #[cfg(feature = "desktop")]
    launch(App);

    #[cfg(feature = "web")]
    launch(App);

    #[cfg(feature = "mobile")]
    dioxus_mobile::launch(App);

    // Fallback if no feature is enabled
    #[cfg(not(any(feature = "desktop", feature = "web", feature = "mobile")))]
    println!("Please enable a feature flag (desktop, web, or mobile) to run the app.");
}

// Main App Component
fn App() -> Element {
    let active_server = use_signal(|| "server1".to_string());
    let active_channel = use_signal(|| "channel1".to_string());

    // Mock data
    let servers = create_mock_servers();

    // Find current server and channel
    let current_server = servers
        .iter()
        .find(|s| s.id == *active_server.get())
        .unwrap_or(&servers[0]);

    let current_channel = current_server
        .channels
        .iter()
        .find(|c| c.id == *active_channel.get() && matches!(c.channel_type, ChannelType::Text))
        .unwrap_or(&current_server.channels[0]);

    // Filter for users in the current server (mock users)
    let users = create_mock_users();

    rsx! {
        div { class: "discord-theme h-screen flex overflow-hidden text-white bg-discord-darkest",
            // Servers Sidebar
            ServerSidebar {
                servers: servers.clone(),
                active_server: active_server
            }

            // Channels Sidebar
            div { class: "w-60 bg-discord-darker flex flex-col",
                // Server Header
                div { class: "h-12 border-b border-gray-900 shadow-sm flex items-center px-4 font-semibold",
                    {current_server.name.clone()}
                }

                // Channel List
                div { class: "flex-1 overflow-y-auto py-2",
                    ChannelList {
                        channels: current_server.channels.clone(),
                        active_channel: active_channel
                    }
                }

                // User Profile
                div { class: "h-14 bg-discord-darker flex items-center px-2 py-2 shadow-lg",
                    div { class: "flex items-center",
                        div { class: "w-8 h-8 rounded-full bg-gray-600 mr-2" }
                        div { class: "text-sm",
                            div { "YourUsername" }
                            div { class: "text-xs text-discord-channelText", "#1234" }
                        }
                    }
                }
            }

            // Main Chat Area
            div { class: "flex-1 flex flex-col bg-discord-dark",
                // Channel Header
                div { class: "h-12 border-b border-gray-900 flex items-center px-4",
                    div { class: "text-lg font-semibold flex items-center",
                        span { class: "text-discord-channelText mr-2", "#" }
                        span { "{current_channel.name}" }
                    }
                }

                // Messages
                ChatMessages {
                    messages: current_channel.messages.clone()
                }

                // Message Input
                MessageInput {}
            }

            // Users Sidebar
            div { class: "w-60 bg-discord-darker hidden md:block",
                div { class: "px-3 h-12 flex items-center shadow-sm border-b border-gray-900",
                    "Members"
                }
                UsersList {
                    users: users
                }
            }
        }
    }
}

// ─────────────  Server sidebar  ──────────────────────────────────────────────
#[component]
pub fn ServerSidebar(servers: Vec<Server>, active_server: Signal<String>) -> Element {
    rsx! {
        div { class: "w-18 bg-discord-darkest flex flex-col items-center pt-3 pb-2 overflow-y-auto",
            /* ----------  Home button  ---------- */
            div { class: "server-icon mb-2 bg-discord-primary", "V" }

            div { class: "w-8 h-0.5 bg-gray-700 rounded-full my-1" }

            /* ----------  Server list  ---------- */
            {servers.into_iter().map(|server| {
                // local data we need inside the closure
                let server_id  = server.id.clone();
                let is_active  = *active_server.get() == server_id;

                // every map‑iteration returns one Node
                rsx!(
                    div {
                        key:   "{server_id}",
                        class: {
                            if is_active {
                                "server-icon active"
                            } else {
                                "server-icon"
                            }
                        },
                        onclick: move |_| active_server.set(server_id.clone()),

                        {
                            // icon or first letter
                            if server.icon.is_empty() {
                                rsx!( span { "{server.name.chars().next().unwrap_or('S')}" } )
                            } else {
                                rsx!( span { "{server.icon}" } )
                            }
                        }
                    }
                )
            })}

            /* ----------  Add server button  ---------- */
            div {
                class:
                    "server-icon mt-2 bg-discord-darker text-discord-primary \
                     hover:bg-discord-primary hover:text-white",
                "+"
            }
        }
    }
}

// ─────────────  Channel list  ────────────────────────────────────────────────
#[component]
pub fn ChannelList(channels: Vec<Channel>, active_channel: Signal<String>) -> Element {
    rsx! {
        div { class: "flex flex-col space-y-1",
            {
                channels
                    .into_iter()
                    .filter(|c| c.channel_type != ChannelType::Category)
                    .map(|channel| {
                        let cid        = channel.id.clone();
                        let is_active  = *active_channel.get() == cid;
                        let icon = match channel.channel_type {
                            ChannelType::Text  => "#",
                            ChannelType::Voice => "🔊",
                            _                  => "",
                        };

                        rsx!(
                            div {
                                key:   "{cid}",
                                class: { if is_active { "channel active" } else { "channel" } },
                                onclick: move |_| active_channel.set(cid.clone()),

                                span { class: "mr-1 text-discord-channelText", "{icon}" }
                                span { "{channel.name}" }
                            }
                        )
                    })
            }
        }
    }
}

// ─────────────  Chat messages  ───────────────────────────────────────────────
#[component]
pub fn ChatMessages(messages: Vec<Message>) -> Element {
    rsx! {
        div { class: "flex-1 overflow-y-auto py-4 px-4 space-y-1",
            {
                /* no messages yet ------------------------------------------------- */
                if messages.is_empty() {
                    rsx!(
                        div { class: "flex flex-col items-center justify-center h-full text-discord-channelText",
                            div { class: "text-lg mb-2", "No messages yet!" }
                            div { "Send a message to start the conversation." }
                        }
                    )
                } else {
                /* render all messages -------------------------------------------- */
                    rsx!(
                        {
                            messages.into_iter().map(|m| {
                                rsx!(
                                    div { key: "{m.id}", class: "flex items-start message py-2",
                                        div { class: "w-10 h-10 rounded-full bg-gray-600 mr-3 flex-shrink-0" }
                                        div { class: "flex-1",
                                            div { class: "flex items-baseline",
                                                span { class: "font-medium mr-2 text-white", "{m.author.username}" }
                                                span { class: "text-xs text-discord-channelText", "{m.timestamp}" }
                                            }
                                            p { class: "text-gray-100 break-words", "{m.content}" }
                                        }
                                    }
                                )
                            })
                        }
                    )
                }
            }
        }
    }
}

// Message Input Component
#[component]
fn MessageInput() -> Element {
    let mut message = use_signal(|| "".to_string());

    let send_message = move |_| {
        if message.get().trim().is_empty() {
            return;
        }
        println!("Sending message: {}", message.get());
        // Here you would typically send the message to your backend
        message.set("".to_string());
    };

    rsx! {
        div { class: "h-24 px-4 pb-6 pt-2",
            div { class: "bg-gray-700 rounded-lg p-1",
                div { class: "flex",
                    // Add attachment button
                    button { class: "p-2 text-gray-400 hover:text-gray-200",
                        "📎"
                    }

                    // Message input
                    input {
                        class: "flex-1 bg-transparent text-gray-100 px-2 py-2 focus:outline-none",
                        placeholder: "Message #channel-name",
                        value: "{message}",
                        oninput: move |evt| message.set(evt.value().clone()),
                        onkeydown: move |evt| {
                            if evt.key() == "Enter" && !evt.modifiers().shift() {
                                evt.stop_propagation();
                                send_message(());
                            }
                        }
                    }

                    // Emoji button
                    button { class: "p-2 text-gray-400 hover:text-gray-200",
                        "😀"
                    }

                    // Send button
                    button {
                        class: "p-2 text-gray-400 hover:text-gray-200",
                        disabled: "{message.get().trim().is_empty()}",
                        onclick: send_message,
                        "↑"
                    }
                }
            }
        }
    }
}

// Users List Component
#[component]
fn UsersList(users: Vec<User>) -> Element {
    // Group users by status
    let online_users: Vec<_> = users
        .iter()
        .filter(|u| matches!(u.status, UserStatus::Online))
        .collect();
    let offline_users: Vec<_> = users
        .iter()
        .filter(|u| matches!(u.status, UserStatus::Offline))
        .collect();

    rsx! {
        div { class: "overflow-y-auto",
            // Online users
            div { class: "px-3 pt-4 pb-2 text-xs text-discord-channelText",
                "ONLINE — {online_users.len()}"
            }

            {online_users.into_iter().map(|user| {
                rsx! {
                    div { class: "px-3 py-1 mx-2 rounded hover:bg-gray-700 flex items-center cursor-pointer", key: "{user.id}",
                        // User avatar with status indicator
                        div { class: "relative mr-2",
                            div { class: "w-8 h-8 rounded-full bg-gray-600" }
                            div {
                                class: "absolute bottom-0 right-0 w-3 h-3 rounded-full border-2 border-discord-darker user-status-online"
                            }
                        }

                        // Username
                        div {
                            class: "text-sm",
                            "{user.username}"
                        }
                    }
                }
            })}

            // Offline users
            div { class: "px-3 pt-4 pb-2 text-xs text-discord-channelText",
                "OFFLINE — {offline_users.len()}"
            }

            {offline_users.into_iter().map(|user| {
                rsx! {
                    div { class: "px-3 py-1 mx-2 rounded hover:bg-gray-700 flex items-center cursor-pointer text-gray-500", key: "{user.id}",
                        // User avatar with status indicator
                        div { class: "relative mr-2",
                            div { class: "w-8 h-8 rounded-full bg-gray-600 opacity-50" }
                            div {
                                class: "absolute bottom-0 right-0 w-3 h-3 rounded-full border-2 border-discord-darker user-status-offline"
                            }
                        }

                        // Username
                        div {
                            class: "text-sm",
                            "{user.username}"
                        }
                    }
                }
            })}
        }
    }
}

// --- Mock Data Functions ---
// Added parent_id to Channel and updated data
fn create_mock_servers() -> Vec<Server> {
    vec![
        Server {
            id: "server1".to_string(),
            name: "Vauxl Official".to_string(),
            icon: "V".to_string(),
            channels: vec![
                Channel { id: "cat1".to_string(), name: "General".to_string(), channel_type: ChannelType::Category, parent_id: None, messages: vec![] },
                Channel {
                    id: "channel1".to_string(),
                    name: "welcome".to_string(),
                    channel_type: ChannelType::Text,
                    parent_id: Some("cat1".to_string()), // Link to category
                    messages: vec![
                        Message {
                            id: "msg1".to_string(),
                            content: "Welcome to Vauxl, the privacy-focused Discord alternative!".to_string(),
                            author: User { id: "user1".to_string(), username: "Admin".to_string(), avatar: "".to_string(), status: UserStatus::Online },
                            timestamp: "Yesterday at 4:30 PM".to_string(),
                        },
                        Message {
                            id: "msg2".to_string(),
                            content: "This is built with Rust and Dioxus! 🦀\nIt uses Tailwind for styling.".to_string(),
                            author: User { id: "user2".to_string(), username: "KyleDerZweite".to_string(), avatar: "".to_string(), status: UserStatus::Online },
                            timestamp: "Yesterday at 4:32 PM".to_string(),
                        },
                        Message {
                            id: "msg3".to_string(),
                            content: "Let me know if you find any bugs.".to_string(),
                            author: User { id: "user1".to_string(), username: "Admin".to_string(), avatar: "".to_string(), status: UserStatus::Online },
                            timestamp: "Today at 9:15 AM".to_string(),
                        },
                    ],
                },
                Channel { id: "channel2".to_string(), name: "announcements".to_string(), channel_type: ChannelType::Text, parent_id: Some("cat1".to_string()), messages: vec![] },
                Channel { id: "cat2".to_string(), name: "Voice Channels".to_string(), channel_type: ChannelType::Category, parent_id: None, messages: vec![] },
                Channel { id: "channel3".to_string(), name: "Lobby".to_string(), channel_type: ChannelType::Voice, parent_id: Some("cat2".to_string()), messages: vec![] },
                Channel { id: "channel4".to_string(), name: "Gaming".to_string(), channel_type: ChannelType::Voice, parent_id: Some("cat2".to_string()), messages: vec![] },
                Channel { id: "channel5".to_string(), name: "uncategorized-text".to_string(), channel_type: ChannelType::Text, parent_id: None, messages: vec![] }, // Uncategorized
            ]
        },
        Server {
            id: "server2".to_string(),
            name: "Rust Developers".to_string(),
            icon: "R".to_string(),
            channels: vec![
                Channel { id: "cat3".to_string(), name: "Development".to_string(), channel_type: ChannelType::Category, parent_id: None, messages: vec![] },
                Channel { id: "channel6".to_string(), name: "help".to_string(), channel_type: ChannelType::Text, parent_id: Some("cat3".to_string()), messages: vec![] },
                Channel { id: "channel7".to_string(), name: "showcase".to_string(), channel_type: ChannelType::Text, parent_id: Some("cat3".to_string()), messages: vec![] },
            ]
        },
        // Add more servers if needed
    ]
}

fn create_mock_users() -> Vec<User> {
    vec![
        User {
            id: "user1".to_string(),
            username: "Admin".to_string(),
            avatar: "".to_string(),
            status: UserStatus::Online,
        },
        User {
            id: "user2".to_string(),
            username: "KyleDerZweite".to_string(),
            avatar: "".to_string(),
            status: UserStatus::Online,
        },
        User {
            id: "user3".to_string(),
            username: "RustEnthusiast".to_string(),
            avatar: "".to_string(),
            status: UserStatus::Idle,
        },
        User {
            id: "user4".to_string(),
            username: "PrivacyFan".to_string(),
            avatar: "".to_string(),
            status: UserStatus::DoNotDisturb,
        },
        User {
            id: "user5".to_string(),
            username: "DioxusUser".to_string(),
            avatar: "".to_string(),
            status: UserStatus::Offline,
        },
        User {
            id: "user6".to_string(),
            username: "TailwindLover".to_string(),
            avatar: "".to_string(),
            status: UserStatus::Offline,
        },
        User {
            id: "user7".to_string(),
            username: "LongUsernameThatMightNeedTruncation".to_string(),
            avatar: "".to_string(),
            status: UserStatus::Online,
        },
        User {
            id: "user8".to_string(),
            username: "AnotherOnlineUser".to_string(),
            avatar: "".to_string(),
            status: UserStatus::Online,
        },
    ]
}
