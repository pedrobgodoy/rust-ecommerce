use async_trait::async_trait;

#[async_trait]
pub trait Command<Input, Output> {
    async fn handle(&self, input: Input) -> Output;
}
