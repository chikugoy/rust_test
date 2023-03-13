#[derive(Debug)]
pub struct FormulaResultData {
    pub id: i64,
    pub value: Option<f64>
}

#[derive(Debug)]
pub struct FormulaResults {
    pub budget: Vec<FormulaResultData>,
    pub row: Vec<FormulaResultData>
}

impl FormulaResults {
    pub fn new() -> Self {
        Self {
            budget: vec![],
            row: vec![]
        }
    }

    pub fn push(&mut self, formula_type: String, value: FormulaResultData) {
        if formula_type.to_string() == "row".to_string() {
            self.row.push(value);
            return;
        }
        if formula_type.to_string() == "budget".to_string() {
            self.budget.push(value);
        }
    }
}
