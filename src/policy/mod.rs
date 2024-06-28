mod block;
mod ostatic;
mod plan;
mod statement;

use std::error::Error;

use serde_derive::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Plans {
    plans: Vec<plan::Plan>,
}

#[derive(Deserialize, Debug)]
pub struct Policy {
    r#static: ostatic::Static,
    plans: Plans,
}

impl Policy {
    pub fn from_json(content: &str) -> Result<Policy, Box<dyn Error>> {
        Ok(serde_json::from_str(content)?)
    }

    pub fn r#static(&self) -> &ostatic::Static {
        &self.r#static
    }

    pub fn plan(&self, index: usize) -> &plan::Plan {
        &self.plans.plans[index]
    }

    pub fn plans(&self) -> impl Iterator<Item = &plan::Plan> {
        self.plans.plans.iter()
    }
}
