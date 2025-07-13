//! Starterm - The GPU Enhanced Terminal.

#![warn(rust_2018_idioms, future_incompatible)]
#![deny(clippy::all, clippy::if_not_else, clippy::enum_glob_use)]
#![cfg_attr(clippy, deny(warnings))]
// With the default subsystem, 'console', windows creates an additional console
// window for the program.
// This is silently ignored on non-windows systems.
// See https://msdn.microsoft.com/en-us/library/4cc7ya5b.aspx for more details.
#![windows_subsystem = "windows"]

#[cfg(not(any(feature = "x11", feature = "wayland", target_os = "macos", windows)))]
compile_error!(r#"at least one of the "x11"/"wayland" features must be enabled"#);

use std::error::Error;
use std::fmt::Write as _;
use std::io::{self, Write};
use std::path::PathBuf;
use std::{env, fs};

use log::info;
#[cfg(windows)]
use windows_sys::Win32::System::Console::{AttachConsole, FreeConsole, ATTACH_PARENT_PROCESS};
use winit::event_loop::EventLoop;
#[cfg(all(feature = "x11", not(any(target_os = "macos", windows))))]
use winit::raw_window_handle::{HasDisplayHandle, RawDisplayHandle};

use starterm_terminal::tty;

mod cli;
mod clipboard;
mod config;
mod daemon;
mod display;
mod event;
mod input;
#[cfg(unix)]
mod ipc;
mod logging;
#[cfg(target_os = "macos")]
mod macos;
mod message_bar;
mod migrate;
#[cfg(windows)]
mod panic;
mod renderer;
mod scheduler;
mod string;
mod ui;
mod window_context;
mod workflows;

mod gl {
    #![allow(clippy::all)]
    include!(concat!(env!("OUT_DIR"), "/gl_bindings.rs"));
}

#[cfg(unix)]
use crate::cli::MessageOptions;
#[cfg(not(any(target_os = "macos", windows)))]
use crate::cli::SocketMessage;
use crate::cli::{Options, Subcommands};
use crate::config::monitor::ConfigMonitor;
use crate::config::UiConfig;
use crate::event::{Event, Processor};
#[cfg(target_os = "macos")]
use crate::macos::locale;

use starterm::ai::core::AiCore;
use starterm::ai::llm::client::LlmClient;
use starterm::ai::llm::openai::OpenAiClient;
use starterm::config::Config;
use std::sync::Arc;

// A dummy client for when no real provider is configured.
struct NoOpLlmClient;
#[async_trait::async_trait]
impl LlmClient for NoOpLlmClient {
    async fn execute_chat(&self, _request: starterm::ai::llm::client::LlmRequest) -> Result<starterm::ai::llm::client::LlmResponse, starterm::ai::llm::client::LlmError> {
        Err(starterm::ai::llm::client::LlmError::ApiError {
            status: 501,
            body: "No LLM provider configured.".to_string(),
        })
    }
}

// A simplified representation of the main application entry point.
#[tokio::main]
async fn main() {
    println!("Starting Starterm...");

    // 1. Load configuration from disk.
    let config = Config::load();

    // 2. Instantiate the appropriate LLM client based on the config.
    let llm_client: Arc<dyn LlmClient> = {
        if let Some(api_key) = config.ai.openai.api_key {
            println!("OpenAI provider configured.");
            Arc::new(OpenAiClient::new(api_key, config.ai.openai.api_url))
        } else {
            println!("No LLM provider configured. AI agent will not be functional.");
            Arc::new(NoOpLlmClient)
        }
    };

    // 3. Create the AiCore and inject the client.
    let ai_core = AiCore::new(llm_client);

    // 4. Start the main application event loop...
    println!("AI Core initialized. Application is ready.");
    // `run_event_loop(ai_core)` would be called here.

    // Example Usage:
    // This demonstrates how the fully-wired system would be used.
    let context = starterm::ai::context::AiContext {
        current_input: "ls -la",
        scrollback: vec!["starterm-macros/", "starterm/"],
        cwd: "/home/user/dev/starterm",
        shell_type: "bash",
        last_exit_code: Some(0),
    };
    
    match ai_core.evaluate_prompt("list all rust files and count them", &context).await {
        Ok(gen_workflow) => {
            println!("\n--- Generated Workflow ---");
            println!("Name: {}", gen_workflow.workflow.name);
            println!("Explanation: {}", gen_workflow.explanation);
            for step in gen_workflow.workflow.steps {
                 println!("Step: {:?}", step);
            }
        }
        Err(e) => eprintln!("\nAI Error: {}", e),
    }
}

/// `msg` subcommand entrypoint.
#[cfg(unix)]
#[allow(unused_mut)]
fn msg(mut options: MessageOptions) -> Result<(), Box<dyn Error>> {
    #[cfg(not(any(target_os = "macos", windows)))]
    if let SocketMessage::CreateWindow(window_options) = &mut options.message {
        window_options.activation_token =
            env::var("XDG_ACTIVATION_TOKEN").or_else(|_| env::var("DESKTOP_STARTUP_ID")).ok();
    }
    ipc::send_message(options.socket, options.message).map_err(|err| err.into())
}

/// Temporary files stored for Starterm.
///
/// This stores temporary files to automate their destruction through its `Drop` implementation.
struct TemporaryFiles {
    #[cfg(unix)]
    socket_path: Option<PathBuf>,
    log_file: Option<PathBuf>,
}

impl Drop for TemporaryFiles {
    fn drop(&mut self) {
        // Clean up the IPC socket file.
        #[cfg(unix)]
        if let Some(socket_path) = &self.socket_path {
            let _ = fs::remove_file(socket_path);
        }

        // Clean up logfile.
        if let Some(log_file) = &self.log_file {
            if fs::remove_file(log_file).is_ok() {
                let _ = writeln!(io::stdout(), "Deleted log file at \"{}\"", log_file.display());
            }
        }
    }
}

/// Run main Starterm entrypoint.
///
/// Creates a window, the terminal state, PTY, I/O event loop, input processor,
/// config change monitor, and runs the main display loop.
fn starterm(mut options: Options) -> Result<(), Box<dyn Error>> {
    // Setup winit event loop.
    let window_event_loop = EventLoop::<Event>::with_user_event().build()?;

    // Initialize the logger as soon as possible as to capture output from other subsystems.
    let log_file = logging::initialize(&options, window_event_loop.create_proxy())
        .expect("Unable to initialize logger");

    info!("Welcome to Starterm");
    info!("Version {}", env!("VERSION"));

    #[cfg(all(feature = "x11", not(any(target_os = "macos", windows))))]
    info!(
        "Running on {}",
        if matches!(
            window_event_loop.display_handle().unwrap().as_raw(),
            RawDisplayHandle::Wayland(_)
        ) {
            "Wayland"
        } else {
            "X11"
        }
    );
    #[cfg(not(any(feature = "x11", target_os = "macos", windows)))]
    info!("Running on Wayland");

    // Load configuration file.
    let config = config::load(&mut options);
    log_config_path(&config);

    // Update the log level from config.
    log::set_max_level(config.debug.log_level);

    // Set tty environment variables.
    tty::setup_env();

    // Set env vars from config.
    for (key, value) in config.env.iter() {
        env::set_var(key, value);
    }

    // Switch to home directory.
    #[cfg(target_os = "macos")]
    env::set_current_dir(home::home_dir().unwrap()).unwrap();

    // Set macOS locale.
    #[cfg(target_os = "macos")]
    locale::set_locale_environment();

    // Create the IPC socket listener.
    #[cfg(unix)]
    let socket_path = if config.ipc_socket() {
        match ipc::spawn_ipc_socket(&options, window_event_loop.create_proxy()) {
            Ok(path) => Some(path),
            Err(err) if options.daemon => return Err(err.into()),
            Err(err) => {
                log::warn!("Unable to create socket: {err:?}");
                None
            },
        }
    } else {
        None
    };

    // Setup automatic RAII cleanup for our files.
    let log_cleanup = log_file.filter(|_| !config.debug.persistent_logging);
    let _files = TemporaryFiles {
        #[cfg(unix)]
        socket_path,
        log_file: log_cleanup,
    };

    // Event processor.
    let mut processor = Processor::new(config, options, &window_event_loop);

    // Start event loop and block until shutdown.
    let result = processor.run(window_event_loop);

    // `Processor` must be dropped before calling `FreeConsole`.
    //
    // This is needed for ConPTY backend. Otherwise a deadlock can occur.
    // The cause:
    //   - Drop for ConPTY will deadlock if the conout pipe has already been dropped
    //   - ConPTY is dropped when the last of processor and window context are dropped, because both
    //     of them own an Arc<ConPTY>
    //
    // The fix is to ensure that processor is dropped first. That way, when window context (i.e.
    // PTY) is dropped, it can ensure ConPTY is dropped before the conout pipe in the PTY drop
    // order.
    //
    // FIXME: Change PTY API to enforce the correct drop order with the typesystem.

    // Terminate the config monitor.
    if let Some(config_monitor) = processor.config_monitor.take() {
        config_monitor.shutdown();
    }

    // Without explicitly detaching the console cmd won't redraw it's prompt.
    #[cfg(windows)]
    unsafe {
        FreeConsole();
    }

    info!("Goodbye");

    result
}

fn log_config_path(config: &UiConfig) {
    if config.config_paths.is_empty() {
        return;
    }

    let mut msg = String::from("Configuration files loaded from:");
    for path in &config.config_paths {
        let _ = write!(msg, "\n  {:?}", path.display());
    }

    info!("{msg}");
}
