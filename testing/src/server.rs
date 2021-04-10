use trillium::Handler;
use trillium_server_common::{Acceptor, Server};
pub struct TestingServer;
pub type Config<A> = trillium_server_common::Config<TestingServer, A>;

#[trillium::async_trait]
impl Server for TestingServer {
    type Transport = crate::TestIo;

    fn run<A, H>(_config: Config<A>, _handler: H)
    where
        A: Acceptor<Self::Transport>,
        H: Handler,
    {
    }

    async fn run_async<A, H>(_config: Config<A>, _handler: H)
    where
        A: Acceptor<Self::Transport>,
        H: Handler,
    {
    }
}

pub fn run(handler: impl Handler) {
    config().run(handler)
}

pub async fn run_async(handler: impl Handler) {
    config().run_async(handler).await
}

pub fn config() -> Config<()> {
    Config::new()
}
