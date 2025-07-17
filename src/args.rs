use clap::{Parser, Subcommand};
#[derive(Debug, Parser)]
#[command(
    name = "lengthgrep",
    version = "1.0",
    about = "human genomics utilities.
       ************************************************
      Gaurav Sablok, IBCH, PAN, Poznan, Poland,
      https://portal.ichb.pl/laboratory-of-genomics/.
      Email: gsablok@ibch.poznan.pl
      Prof. Luiza Handschuh
      Email: luizahan@ibch.poznan.pl.
      ************************************************"
)]
pub struct CommandParse {
    /// subcommands for the specific actions
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// threaded version of genelength for human
    ThreadedLengthHuman {
        /// provide yes as argument
        generate: String,
    },
    /// trhreaded version of genelength for mouse
    ThreadedLengthMouse {
        /// provide yes as argument
        generate: String,
    },
}
