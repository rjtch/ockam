use miette::IntoDiagnostic;
use ockam_abac::{Action, Resource};

use ockam_core::api::Request;
use ockam_core::async_trait;
use ockam_node::Context;

use crate::cloud::Controller;
use crate::nodes::models::policy::{Expression, Policy};

const TARGET: &str = "ockam_api::cloud::policy";

#[async_trait]
pub trait ControllerPolicy {
    async fn create_policy(
        &self,
        ctx: &Context,
        node_name: String,
        resource: String,
        action: String,
        bdy: Policy,
    ) -> miette::Result<Expression>;

    async fn show_policy(&self,
                         ctx: &Context,
                         node_name: String,
                         path: String) -> miette::Result<Expression>;

    async fn list_policies(&self,
                           ctx: &Context,
                           node_name: String,
                           resource: String) -> miette::Result<Vec<Expression>>;
    async fn delete_policy() -> miette::Result<()>;
}

#[async_trait]
impl ControllerPolicy for Controller {
    async fn create_policy(&self,
                           ctx: &Context,
                           node_name: String,
                           resource: String,
                           action: String,
                           bdy: Policy) -> miette::Result<Expression> {
        trace!(target: TARGET, %node_name, %resource, %action, "creating policy");
        let req = Request::post(format!("/policy/{resource}/{action}"))
            .body(bdy);
        self.0
            .ask(ctx, "projects", req)
            .await
            .into_diagnostic()?
            .success()
            .into_diagnostic()
    }

    async fn show_policy(&self,
                         ctx: &Context,
                         node_name: String,
                         path: String) -> miette::Result<Expression> {
        trace!(target: TARGET, %node_name, %path, "Showing Policy");
        let req = Request::get(path);
        self.0
            .ask(ctx, "policy", req)
            .await
            .into_diagnostic()?
            .success()
            .into_diagnostic()
    }

    async fn list_policies(&self,
                           ctx: &Context,
                           node_name: String,
                           resource: String) -> miette::Result<Vec<Expression>> {
        trace!(target: TARGET, %node_name, %resource, "Listing Policies");
        let req = Request::get(format!("/policy/{resource}"));
        self.0
            .ask(ctx, "policies", req)
            .await
            .into_diagnostic()?
            .success()
            .into_diagnostic()
    }

    async fn delete_policy() -> miette::Result<()> {
        todo!()
    }
}
