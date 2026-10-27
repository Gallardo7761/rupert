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

#[derive(Debug, PartialEq)]
pub enum HierarchyError {
    SelfTarget,
    TargetIsBot,
    TargetIsOwner,
    TargetTooHigh,
    BotTooLow,
    RoleTooHigh,
    BotRoleTooLow,
}

impl fmt::Display for HierarchyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            HierarchyError::SelfTarget => write!(f, "No puedes usar esto contigo mismo"),
            HierarchyError::TargetIsBot => write!(f, "No puedes usar esto conmigo"),
            HierarchyError::TargetIsOwner => {
                write!(f, "No puedes usar esto con el dueño del servidor")
            }
            HierarchyError::TargetTooHigh => {
                write!(f, "Su rol más alto es igual o superior al tuyo")
            }
            HierarchyError::BotTooLow => write!(f, "Su rol más alto es igual o superior al mío"),
            HierarchyError::RoleTooHigh => {
                write!(f, "Ese rol es igual o superior a tu rol más alto")
            }
            HierarchyError::BotRoleTooLow => {
                write!(f, "Ese rol es igual o superior a mi rol más alto")
            }
        }
    }
}

impl std::error::Error for HierarchyError {}

#[derive(Debug)]
pub struct GifError(nekosbest::NekosBestError);

impl fmt::Display for GifError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "No se pudo conseguir el GIF")
    }
}

impl std::error::Error for GifError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        Some(&self.0)
    }
}

impl From<nekosbest::NekosBestError> for GifError {
    fn from(e: nekosbest::NekosBestError) -> Self {
        Self(e)
    }
}

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
