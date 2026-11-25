use std::ops::ControlFlow;

pub fn prompt_for_input<'buf>(
    prompt: &str,
    buf: &'buf mut String,
    input: &mut impl std::io::BufRead,
    prompt_output: &mut impl std::io::Write,
) -> std::io::Result<ControlFlow<(), &'buf str>> {
    write!(prompt_output, "{prompt}")?;
    prompt_output.flush()?;
    buf.clear();
    let n = input.read_line(buf)?;
    if n == 0 {
        return Ok(ControlFlow::Break(()));
    }
    Ok(ControlFlow::Continue(&buf[..n]))
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn prompt_with_input() {
        const PROMPT: &str = "$ ";
        let mut buf = String::new();
        let input = String::from("hello");
        let mut prompt_output = [0_u8; PROMPT.len()];

        let control_flow = prompt_for_input(
            PROMPT,
            &mut buf,
            &mut input.as_bytes(),
            &mut prompt_output.as_mut_slice(),
        )
        .unwrap();

        assert!(control_flow.is_continue());
        assert_eq!(buf, "hello");
        assert_eq!(&prompt_output, b"$ ");
    }

    #[test]
    fn prompt_without_input() {
        const PROMPT: &str = "$ ";
        let mut buf = String::new();
        let input = String::from("");
        let mut prompt_output = [0_u8; PROMPT.len()];

        let control_flow = prompt_for_input(
            PROMPT,
            &mut buf,
            &mut input.as_bytes(),
            &mut prompt_output.as_mut_slice(),
        )
        .unwrap();

        assert!(control_flow.is_break());
        assert_eq!(&prompt_output, b"$ ");
    }
}
