use clap::Args;

use ockam::Context;
use ockam_abac::{Action, Resource};
use ockam_api::address::extract_address_value;
use ockam_api::nodes::InMemoryNode;
use ockam_core::api::Request;

use crate::CommandGlobalOpts;
use ockam_api::cloud::policy::ControllerPolicy;
use crate::policy::policy_path;
use crate::util::node_rpc;

#[derive(Clone, Debug, Args)]
pub struct ShowCommand {
    #[arg(long, display_order = 900, id = "NODE_NAME")]
    at: String,

    #[arg(short, long)]
    resource: Resource,

    #[arg(short, long)]
    action: Action,
}

impl ShowCommand {
    pub fn run(self, options: CommandGlobalOpts) {
        node_rpc(rpc, (options, self));
    }
}

async fn rpc(ctx: Context, (opts, cmd): (CommandGlobalOpts, ShowCommand)) -> miette::Result<()> {
    run_impl(&ctx, opts, cmd).await
}

async fn run_impl(ctx: &Context, opts: CommandGlobalOpts, cmd: ShowCommand) -> miette::Result<()> {
    let node_name = extract_address_value(&cmd.at)?;
    let node = InMemoryNode::start(ctx, &opts.state).await?;
    let controller = node.create_controller().await?;

    let path = policy_path(&cmd.resource, &cmd.action);
    let policy = controller.show_policy(
        ctx,
        node_name,
        path).await?;
    println!("{:?}", policy);
    Ok(())
}
