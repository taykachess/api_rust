// Stops the client from outputing a huge number of warnings during compilation.
#[allow(warnings, unused)]
mod prisma;

use prisma::PrismaClient;
use prisma_client_rust::NewClientError;

async fn init_bd() {
    let prisma = PrismaClient::_builder()
        .build()
        .await
        .expect("Connection is closed");
}
