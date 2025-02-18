use clap::Parser;
mod cli;
use cli::Args;
use cli::Commands;
mod todo;
use todo::add_item;
use todo::list_items;
use todo::remove_item;
use todo::complete_item;
use todo::clear_items;

fn main() {
    let args = Args::parse();
    match args.command {
        Commands::Add { item } => add_item(item),
        Commands::Remove { itemid } => remove_item(itemid),
        Commands::Complete {itemid} => complete_item(itemid),
        Commands::List => list_items(),
        Commands::Clear => clear_items()
    }
}
