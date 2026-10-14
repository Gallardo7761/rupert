use std::fmt;

#[derive(Debug, PartialEq)]
pub enum CalcError {
    Empty,
    TooLong { max: usize },
    Syntax { error: String },
    Evaluation { error: String },
    NotFinite,
    UnitsNotSupported,
}

impl fmt::Display for CalcError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            CalcError::Empty => write!(f, "La operación está vacía"),
            CalcError::TooLong { max } => {
                write!(f, "Operación demasiado larga. Máximo {max} caracteres")
            }
            CalcError::Syntax { error } => write!(f, "Error de sintaxis: {error}"),
            CalcError::Evaluation { error } => write!(f, "No se pudo calcular: {error}"),
            CalcError::NotFinite => write!(f, "El resultado no es un número finito"),
            CalcError::UnitsNotSupported => write!(f, "De momento no se admiten unidades"),
        }
    }
}

impl std::error::Error for CalcError {}

impl From<mathengine::Error> for CalcError {
    fn from(e: mathengine::Error) -> Self {
        match e {
            mathengine::Error::Lexer(e) => Self::Syntax {
                error: e.to_string(),
            },
            mathengine::Error::Parser(e) => Self::Syntax {
                error: e.to_string(),
            },
            mathengine::Error::Evaluator(e) => Self::Evaluation {
                error: e.to_string(),
            },
        }
    }
}
