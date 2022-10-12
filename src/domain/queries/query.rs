use async_trait::async_trait;

#[async_trait]
pub trait Query<Input, Output> {
    async fn handle(&self, input: Input) -> Output;
}
