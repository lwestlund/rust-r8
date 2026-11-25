use std::num::NonZeroUsize;

#[derive(Debug, PartialEq)]
pub enum Instruction {
    Step { count: NonZeroUsize },
}

impl Instruction {
    const DEFAULT_STEP: NonZeroUsize = NonZeroUsize::new(1).unwrap();

    pub fn parse(line: &str) -> Result<Option<Self>, InstructionParseError> {
        let mut words = line.split_whitespace();
        match words.next() {
            Some(word) if !word.is_empty() => match word {
                "step" | "s" => {
                    let count = words
                        .next()
                        .and_then(|count| count.parse().ok())
                        .unwrap_or(Self::DEFAULT_STEP);
                    Ok(Some(Self::Step { count }))
                }
                _ => Err(InstructionParseError::UnknownInstruction(word.to_string())),
            },
            Some(_) | None => Ok(None),
        }
    }
}

#[derive(Debug)]
pub enum InstructionParseError {
    UnknownInstruction(String),
}

impl std::fmt::Display for InstructionParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            InstructionParseError::UnknownInstruction(s) => write!(f, "unknown instruction: {s}"),
        }
    }
}

impl std::error::Error for InstructionParseError {}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn parse_instruction_step() {
        let text = "step";

        let instruction = Instruction::parse(text).unwrap();

        assert_eq!(
            instruction,
            Some(Instruction::Step {
                count: 1.try_into().unwrap()
            })
        );
    }

    #[test]
    fn parse_instruction_step_short() {
        let text = "s";

        let instruction = Instruction::parse(text).unwrap();

        assert_eq!(
            instruction,
            Some(Instruction::Step {
                count: 1.try_into().unwrap()
            })
        );
    }

    #[test]
    fn parse_instruction_step_with_count() {
        let text = "step 4";

        let instruction = Instruction::parse(text).unwrap();

        assert_eq!(
            instruction,
            Some(Instruction::Step {
                count: 4.try_into().unwrap()
            })
        );
    }

    #[test]
    fn parse_instruction_step_short_with_count() {
        let text = "s 4";

        let instruction = Instruction::parse(text).unwrap();

        assert_eq!(
            instruction,
            Some(Instruction::Step {
                count: 4.try_into().unwrap()
            })
        );
    }

    #[test]
    fn parse_instruction_step_with_many_whitespace_before_count() {
        let text = "step      4";

        let instruction = Instruction::parse(text).unwrap();

        assert_eq!(
            instruction,
            Some(Instruction::Step {
                count: 4.try_into().unwrap()
            })
        );
    }

    #[test]
    fn parse_unknown_instruction() {
        let text = "not-an-instruction";

        let instruction = Instruction::parse(text);

        assert!(instruction.is_err());
    }

    /// An empty instruction is useful to detect that the user gave us no input, which can carry
    /// special meaning like wanting to execute the last instruction again.
    #[test]
    fn parse_empty_instruction() {
        let text = "";

        let instruction = Instruction::parse(text).unwrap();

        assert!(instruction.is_none());
    }
}
