use crate::input::Lambda;
use aws_config::{BehaviorVersion, Region};
use aws_sdk_lambda::{
    Client,
    error::SdkError,
    operation::{
        add_permission::{AddPermissionError, AddPermissionOutput},
        create_function::{
            CreateFunctionError, CreateFunctionOutput, builders::CreateFunctionFluentBuilder,
        },
    },
    types::{FunctionCode, Runtime},
};

pub struct LambdaClient {
    client: Client,
}

impl LambdaClient {
    pub async fn new() -> Self {
        let config = aws_config::defaults(BehaviorVersion::latest())
            .region(Region::new("eu-west-1"))
            .load()
            .await;
        LambdaClient {
            client: Client::new(&config),
        }
    }

    pub async fn create_lambda(
        &self,
        lambda: &Lambda,
    ) -> Result<CreateFunctionOutput, SdkError<CreateFunctionError>> {
        let builder = self.create_lambda_builder(lambda);
        builder.send().await
    }

    fn create_lambda_builder(&self, lambda: &Lambda) -> CreateFunctionFluentBuilder {
        let mut builder = self
            .client
            .create_function()
            .function_name(&lambda.name)
            .role("arn:aws:iam:2624382231998:role/lambda-basic-permissions-role")
            .code(
                FunctionCode::builder()
                    .s3_bucket("lambda-sam-van-overmeire")
                    .s3_key("example.zip")
                    .build(),
            )
            .runtime(Runtime::Nodejs18x)
            .handler("handler.handler");

        if let Some(time) = lambda.timeout {
            builder = builder.timeout(time.into());
        }
        if let Some(memory) = lambda.memory {
            builder = builder.memory_size(memory.into());
        }

        builder
    }

    pub async fn add_bucket_permission(
        &self,
        lambda: &Lambda,
        bucket_name: &str,
    ) -> Result<AddPermissionOutput, SdkError<AddPermissionError>> {
        self.client
            .add_permission()
            .function_name(&lambda.name)
            .principal("*")
            .statement_id("StatementId")
            .action("lambda:InvokeFunction")
            .source_arn(format!("arn:aws:s3:::{}", bucket_name))
            .send()
            .await
    }
}
