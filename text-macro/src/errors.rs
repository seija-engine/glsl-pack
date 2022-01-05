#[derive(Debug)]
pub enum ParseError {
    ErrIf,
    ErrMacro(String),
    MissFuncBody(String)
}