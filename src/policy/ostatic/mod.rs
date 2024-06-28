use serde_derive::Deserialize;

#[derive(Deserialize, Debug)]
pub struct StringConstant {
    value: String,
}

impl StringConstant {
    pub fn value(&self) -> &str {
        &self.value
    }
}

#[derive(Deserialize, Debug)]
pub struct FileConstant {
    value: String,
}

#[derive(Deserialize, Debug)]
pub struct Static {
    strings: Vec<StringConstant>,
    files: Vec<FileConstant>,
}

impl Static {
    pub fn string(&self, index: usize) -> &StringConstant {
        &self.strings[index]
    }

    pub fn strings(&self) -> impl Iterator<Item = &StringConstant> {
        self.strings.iter()
    }
}
