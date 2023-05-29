// Stops the client from outputing a huge number of warnings during compilation.
#[allow(warnings, unused)]
mod prisma;

use std::sync::Arc;

use prisma::PrismaClient;
use prisma_client_rust::NewClientError;

pub async fn init_bd() -> Arc<PrismaClient> {
    let prisma = PrismaClient::_builder()
        .build()
        .await
        .expect("Connection is closed");

    Arc::new(prisma)
}
