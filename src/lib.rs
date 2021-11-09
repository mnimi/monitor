//! `automan` is a support library for the Monitor program.
//!
//! The Monitor is a simple Discord bot that hangs out in the [Narwhals] server.
//! The bot offers several chat games as well as tools to automatically perform certain
//! administrative tasks in order to better keep the server organized.
//!
//! You can add the bot to your own server [here], or you can build your own by following
//! [these instructions].
//!
//! [Narwhals]: https://discord.gg/GyXtwnBWne
//! [here]: https://discord.com/api/oauth2/authorize?client_id=817894435299262516&permissions=8&redirect_uri=https%3A%2F%2Fdiscord.com%2Fapi%2Foauth2%2Fauthorize%3Fclient_id%3D817894435299262516%26scope%3Dapplications.commands&scope=bot
//! [these instructions]: https://github.com/mnimi/monitor/#installation
#![crate_name = "automan"]
#![crate_type = "lib"]
#![deny(clippy::all)]
#![warn(missing_docs)]
#![allow(unused)]
#![allow(clippy::needless_return)]
#![allow(dead_code)]
#![feature(decl_macro)]
#![feature(path_try_exists)]

/// The maximum number of threads allowed to remain activate at once.
pub const MAX_THREADS: usize = 3;

/// Set up logging functionality for the Monitor application.
pub fn setup_logging(level: LevelFilter, logfile: &str) -> Result<(), fern::InitError>
{
   Dispatch::new()
      .format(|out, message, record| {
         out.finish(format_args!(
            "{}[{}][{}] {}",
            Utc::now().format("[%Y-%m-%d] [%HH:%Mm:%Ss]"),
            record.target(),
            record.level(),
            message,
         ))
      })
      .level(level)
      .chain(std::io::stdout())
      .chain(fern::log_file(logfile)?)
      .apply()?;

   return Ok(());
}

/// Sets up the Discord bot.
pub async fn setup_discord(config: &DiscordConfig) -> Result<DiscordBot, GenericError>
{
   // Setup the Discord bot itself.
   let mut bot: DiscordBot = DiscordBot::new(config).await.unwrap();
   if let Err(e) = bot.run().await {
      eprintln!("An error occurred while running the Discord bot!");

      // an error occurred while attempting to run the bot.
      return Err(e.into());
   }

   // all is okay!
   return Ok(bot);
}

/// Sets up the Matrix bot.
pub fn setup_matrix(config: &MatrixConfig) -> Result<MatrixBot, GenericError>
{
   let mut bot: MatrixBot = MatrixBot::new(config).unwrap();

   return Ok(bot);
}

/// START runs the bot.
pub async fn start(options: AppConfig, mut discord: DiscordBot, mut matrix: MatrixBot) -> Result<(), GenericError>
{
   let app_id: String = options.id();

   log::info!("Application: MONITOR");
   log::info!("Version: v0.1.0");
   log::info!("Support: AUTOMAN-0.0.1");
   log::info!("Application ID: {}", app_id);

   log::warn!("---");
   log::warn!("PLEASE NOTE THAT THIS IS A VERY EARLY PRE-RELEASE VERSION OF THE PROGRAM.");
   log::warn!("ALMOST EVERYTHING IN THIS APPLICATION IS SUBJECT TO CHANGE IN THE FUTURE.");
   log::warn!("---");

   let mut childs = vec![];

   let child1: JoinHandle<_> = tokio::spawn(async move {
      discord.run().await;
   });

   let child2: JoinHandle<_> = tokio::spawn(async move {
      matrix.run().await;
   });

   match childs.len() {
      MAX_THREADS => return Err(OOBError.into()),
      0 => childs.push(child1),
      1 => childs.push(child2),
      2 => {}
      _ => return Err(OOBError.into()),
   }

   for child in childs {
      let _ = child.await?;
   }

   return Ok(());
}

// IMPORTS //////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

pub use crate::{
   commands::CommandCounter,
   discord::app::ShardManagerContainer,
   errors::GenericError,
   shared::AppConfig,
};

use self::{
   discord::{
      DiscordBot,
      config::DiscordConfig,
   },
   errors::*,
   matrix::{
      MatrixBot,
      config::MatrixConfig,
   },
};

use std::error::Error;
use tokio::task::JoinHandle;

use chrono::Utc;
use fern::Dispatch;
use log::LevelFilter;

use ulid::Ulid;

// CRATE MODULES ////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

/// Contains bot commands called on Discord or Matrix.
///
/// Commands are prefixed with `mntr` and can be enumerated via
/// the `mntr help` command.
pub mod commands;

/// Contains an implementation of a Discord client via [Serenity].
///
/// [Serenity]: https://github.com/serenity-rs/serenity
pub mod discord;

/// Contains error types.
pub mod errors;

/// Contains light implementations of various chat games.
pub mod games;

/// Contains a GIPHY client implementation.
pub mod gif;

/// Macros for basic utilities in the Monitor.
pub mod macros;

/// Contains an implementation of a Matrix bot and client.
pub mod matrix;

/// Contains globally accessible configuration details.
pub mod shared;

// DEPENDENCIES /////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

extern crate chrono;
extern crate fern;
extern crate rand;
extern crate rillrate;
extern crate serde;
extern crate serenity;
extern crate tokio;
extern crate toml;
extern crate ulid;

#[macro_use]
extern crate lazy_static;

#[macro_use]
extern crate log;

#[macro_use]
extern crate serde_derive;
