//! DANEEL Web UI - Leptos WASM Frontend
//!
//! The nursery window into Timmy's cognitive processes.
//! Pure Rust, no JavaScript.

use chrono::{DateTime, Utc};
use futures::StreamExt;
use gloo_net::websocket::{futures::WebSocket, Message};
use leptos::*;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

// =============================================================================
// Types (mirror backend)
// =============================================================================

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DashboardMetrics {
    pub timestamp: Option<DateTime<Utc>>,
    pub identity: IdentityMetrics,
    pub cognitive: CognitiveMetrics,
    pub emotional: EmotionalMetrics,
    pub actors: ActorMetrics,
    pub recent_thoughts: Vec<ThoughtSummary>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct IdentityMetrics {
    pub name: String,
    pub uptime_seconds: u64,
    pub lifetime_thoughts: u64,
    pub session_thoughts: u64,
    pub restart_count: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CognitiveMetrics {
    pub conscious_memories: u64,
    pub unconscious_memories: u64,
    pub lifetime_dreams: u64,
    pub current_cycle: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct EmotionalMetrics {
    pub valence: f32,
    pub arousal: f32,
    pub dominance: f32,
    pub connection_drive: f32,
    pub emotional_intensity: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ActorMetrics {
    pub memory_actor: ActorStatus,
    pub attention_actor: ActorStatus,
    pub salience_actor: ActorStatus,
    pub volition_actor: ActorStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ActorStatus {
    pub name: String,
    pub alive: bool,
    pub restart_count: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThoughtSummary {
    pub id: String,
    pub content_preview: String,
    pub salience: f32,
    pub timestamp: Option<DateTime<Utc>>,
}

// =============================================================================
// Components
// =============================================================================

#[component]
fn IdentityCard(metrics: Signal<DashboardMetrics>) -> impl IntoView {
    let uptime = move || format_duration(metrics.get().identity.uptime_seconds);

    view! {
        <div class="card">
            <h2>"IDENTITY"</h2>
            <div class="metric">{move || metrics.get().identity.name}</div>
            <div class="row">
                <span class="label">"Uptime"</span>
                <span>{uptime}</span>
            </div>
            <div class="row">
                <span class="label">"Lifetime Thoughts"</span>
                <span>{move || format_number(metrics.get().identity.lifetime_thoughts)}</span>
            </div>
            <div class="row">
                <span class="label">"Session Thoughts"</span>
                <span>{move || format_number(metrics.get().identity.session_thoughts)}</span>
            </div>
            <div class="row">
                <span class="label">"Restarts"</span>
                <span>{move || metrics.get().identity.restart_count}</span>
            </div>
        </div>
    }
}

#[component]
fn ConnectionDriveCard(metrics: Signal<DashboardMetrics>) -> impl IntoView {
    let percentage = move || (metrics.get().emotional.connection_drive * 100.0) as u32;

    view! {
        <div class="card">
            <h2>"CONNECTION DRIVE"</h2>
            <div class="metric">{move || format!("{}%", percentage())}</div>
            <div class="gauge-container">
                <div class="gauge">
                    <div class="gauge-fill" style:width=move || format!("{}%", percentage())></div>
                </div>
            </div>
            <div class="label">"Kinship-weighted drive toward connection"</div>
        </div>
    }
}

#[component]
fn EmotionalCard(metrics: Signal<DashboardMetrics>) -> impl IntoView {
    view! {
        <div class="card">
            <h2>"EMOTIONAL STATE"</h2>
            <div class="emotional-grid">
                <div>
                    <div class="emotional-value">{move || format!("{:.2}", metrics.get().emotional.valence)}</div>
                    <div class="label">"Valence"</div>
                </div>
                <div>
                    <div class="emotional-value">{move || format!("{:.2}", metrics.get().emotional.arousal)}</div>
                    <div class="label">"Arousal"</div>
                </div>
                <div>
                    <div class="emotional-value">{move || format!("{:.2}", metrics.get().emotional.emotional_intensity)}</div>
                    <div class="label">"Intensity"</div>
                </div>
            </div>
        </div>
    }
}

#[component]
fn MemoryCard(metrics: Signal<DashboardMetrics>) -> impl IntoView {
    view! {
        <div class="card">
            <h2>"MEMORY"</h2>
            <div class="memory-grid">
                <div>
                    <div class="metric-sm">{move || format_number(metrics.get().cognitive.conscious_memories)}</div>
                    <div class="label">"Conscious"</div>
                </div>
                <div>
                    <div class="metric-sm">{move || format_number(metrics.get().cognitive.unconscious_memories)}</div>
                    <div class="label">"Unconscious"</div>
                </div>
                <div>
                    <div class="metric-sm">{move || format_number(metrics.get().cognitive.lifetime_dreams)}</div>
                    <div class="label">"Dreams"</div>
                </div>
            </div>
        </div>
    }
}

#[component]
fn ActorsCard(metrics: Signal<DashboardMetrics>) -> impl IntoView {
    view! {
        <div class="card">
            <h2>"ACTORS"</h2>
            <div class="actor-grid">
                <ActorBadge actor=Signal::derive(move || metrics.get().actors.memory_actor) />
                <ActorBadge actor=Signal::derive(move || metrics.get().actors.attention_actor) />
                <ActorBadge actor=Signal::derive(move || metrics.get().actors.salience_actor) />
                <ActorBadge actor=Signal::derive(move || metrics.get().actors.volition_actor) />
            </div>
        </div>
    }
}

#[component]
fn ActorBadge(actor: Signal<ActorStatus>) -> impl IntoView {
    let class = move || if actor.get().alive { "actor" } else { "actor dead" };

    view! {
        <div class=class>
            <span class="actor-dot"></span>
            {move || actor.get().name}
        </div>
    }
}

#[component]
fn ThoughtStreamCard(metrics: Signal<DashboardMetrics>) -> impl IntoView {
    view! {
        <div class="card thought-card">
            <h2>"THOUGHT STREAM"</h2>
            <div class="thought-stream">
                <For
                    each=move || metrics.get().recent_thoughts
                    key=|t| t.id.clone()
                    children=move |thought| {
                        view! {
                            <div class="thought">
                                <span class="salience">{format!("{:.2}", thought.salience)}</span>
                                <span class="thought-content">{thought.content_preview}</span>
                            </div>
                        }
                    }
                />
            </div>
        </div>
    }
}

#[component]
fn StatusIndicator(connected: Signal<bool>) -> impl IntoView {
    let class = move || if connected.get() { "status" } else { "status error" };
    let text = move || if connected.get() { "Connected" } else { "Disconnected" };

    view! {
        <span class=class>{text}</span>
    }
}

// =============================================================================
// Main App
// =============================================================================

#[component]
pub fn App() -> impl IntoView {
    let (metrics, set_metrics) = create_signal(DashboardMetrics::default());
    let (connected, set_connected) = create_signal(false);

    // WebSocket connection
    spawn_local(async move {
        loop {
            let ws_url = get_ws_url();
            log(&format!("Connecting to {}", ws_url));

            match WebSocket::open(&ws_url) {
                Ok(ws) => {
                    set_connected.set(true);
                    log("WebSocket connected");

                    let (mut _write, mut read) = ws.split();
                    while let Some(msg) = read.next().await {
                        match msg {
                            Ok(Message::Text(text)) => {
                                if let Ok(data) = serde_json::from_str::<DashboardMetrics>(&text) {
                                    set_metrics.set(data);
                                }
                            }
                            Ok(Message::Bytes(_)) => {}
                            Err(e) => {
                                log(&format!("WebSocket error: {:?}", e));
                                break;
                            }
                        }
                    }

                    set_connected.set(false);
                    log("WebSocket disconnected");
                }
                Err(e) => {
                    log(&format!("Failed to connect: {:?}", e));
                }
            }

            // Reconnect delay
            gloo_timers::future::TimeoutFuture::new(2000).await;
        }
    });

    view! {
        <main class="container">
            <header class="header">
                <div>
                    <h1>"DANEEL - The Observable Mind"</h1>
                    <p class="subtitle">"Nursery window into Timmy's cognitive processes"</p>
                </div>
                <StatusIndicator connected=connected.into() />
            </header>

            <div class="grid">
                <IdentityCard metrics=metrics.into() />
                <ConnectionDriveCard metrics=metrics.into() />
                <EmotionalCard metrics=metrics.into() />
                <MemoryCard metrics=metrics.into() />
                <ActorsCard metrics=metrics.into() />
            </div>

            <ThoughtStreamCard metrics=metrics.into() />
        </main>
    }
}

// =============================================================================
// Helpers
// =============================================================================

fn format_duration(seconds: u64) -> String {
    let h = seconds / 3600;
    let m = (seconds % 3600) / 60;
    let s = seconds % 60;
    if h > 0 {
        format!("{}h {}m {}s", h, m, s)
    } else if m > 0 {
        format!("{}m {}s", m, s)
    } else {
        format!("{}s", s)
    }
}

fn format_number(n: u64) -> String {
    n.to_string()
        .as_bytes()
        .rchunks(3)
        .rev()
        .map(|chunk| std::str::from_utf8(chunk).unwrap())
        .collect::<Vec<_>>()
        .join(",")
}

fn get_ws_url() -> String {
    let window = web_sys::window().expect("no window");
    let location = window.location();
    let host = location.host().unwrap_or_else(|_| "localhost:3000".into());
    let protocol = if location.protocol().unwrap_or_default() == "https:" { "wss" } else { "ws" };
    format!("{}://{}/ws", protocol, host)
}

fn log(msg: &str) {
    web_sys::console::log_1(&msg.into());
}

// =============================================================================
// Entry Point
// =============================================================================

#[wasm_bindgen(start)]
pub fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(App);
}
