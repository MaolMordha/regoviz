use serde_derive::Deserialize;

use super::block::Block;

#[derive(Deserialize, Debug)]
pub struct Plan {
    name: String,
    #[serde(default)]
    blocks: Vec<Block>,
}

impl Plan {
    fn name(&self) -> &str {
        &self.name
    }

    fn block(&self, index: usize) -> &Block {
        &self.blocks[index]
    }

    fn blocks(&self) -> impl Iterator<Item = &Block> {
        self.blocks.iter()
    }
}

#[cfg(test)]
mod tests {
    use super::Plan;

    #[test]
    fn can_deserialize_without_blocks_member() {
        let plan: Plan = serde_json::from_str(
            r#"
            {
                "name": "test_plan"
            }
            "#,
        )
        .unwrap();

        assert_eq!(plan.name, "test_plan");
        assert_eq!(plan.blocks.len(), 0);
    }

    #[test]
    fn can_deserialize_with_empty_blocks() {
        let plan: Plan = serde_json::from_str(
            r#"
            {
                "name": "test_plan",
                "blocks": []
            }
            "#,
        )
        .unwrap();

        assert_eq!(plan.blocks.len(), 0);
    }

    #[test]
    fn name_returns_name_field() {
        let plan = Plan {
            name: "test_plan".to_string(),
            blocks: vec![],
        };

        assert_eq!(plan.name(), &plan.name);
    }

    // TODO block and blocks tests
}
