mod handler;
mod methods;
mod protocol;
mod rpc;

use anyhow::Result;

fn main() -> Result<()> {
    rpc::run()
}
