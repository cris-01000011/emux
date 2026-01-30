use color_eyre::Result;

mod actions;
mod app;
mod components;
mod config;
mod ui;
mod utils;

use crate::app::App;

async fn tokio_main() -> Result<()> {
    color_eyre::install()?;

    let terminal = ratatui::init();

    App::new().run(terminal).await?;

    ratatui::restore();

    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    if let Err(e) = tokio_main().await {
        eprintln!("{} error: Something went wrong", env!("CARGO_PKG_NAME"));
        Err(e)
    } else {
        Ok(())
    }
}
