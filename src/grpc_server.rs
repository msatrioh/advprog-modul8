use tonic::{transport::Server, Request, Response, Status};

pub mod services {
    tonic::include_proto!("services");
}

use services::{payment_service_server::{PaymentService, PaymentServiceServer}, PaymentRequest, PaymentResponse};

#[derive(Default)]
pub struct MyPaymentService {}

#[tonic::async_trait]
impl PaymentService for MyPaymentService {
    async fn process_payment(
        &self,
        request: Request<PaymentRequest>,
    ) -> Result<Response<PaymentResponse>, Status> {
        println!("Received payment request: {:?}", request);

        // Process the request and return a response
        // This example immediately returns a success result for demonstration purposes
        Ok(Response::new(PaymentResponse {
            success: true }))
    }
}