mod handler;
mod methods;
mod protocol;
mod rpc;
mod utils;

use anyhow::Result;

fn main() -> Result<()> {
    rpc::run()
}
