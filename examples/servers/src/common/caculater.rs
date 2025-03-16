use rmcp::{ServerHandler, model::ServerInfo, schemars, tool, tool_box};

#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct SumRequest {
    #[schemars(description = "the left hand side number")]
    pub a: i32,
    pub b: i32,
}
#[derive(Debug, Clone)]
pub struct Calculater;
impl Calculater {
    #[tool(description = "Calculate the sum of two numbers")]
    fn sum(&self, #[tool(aggr)] SumRequest { a, b }: SumRequest) -> String {
        (a + b).to_string()
    }

    #[tool(description = "Calculate the sum of two numbers")]
    fn sub(
        &self,
        #[tool(param)]
        #[schemars(description = "the left hand side number")]
        a: i32,
        #[tool(param)]
        #[schemars(description = "the left hand side number")]
        b: i32,
    ) -> String {
        (a - b).to_string()
    }

    tool_box!(Calculater { sum, sub });
}

impl ServerHandler for Calculater {
    tool_box!(@derive);
    fn get_info(&self) -> ServerInfo {
        ServerInfo {
            instructions: Some("A simple caculator".into()),
            ..Default::default()
        }
    }
}
