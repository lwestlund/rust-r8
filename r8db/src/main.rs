use std::ops::ControlFlow;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cpu = r8::Cpu::default();

    let mut debugger = r8db::R8db::new(cpu);

    let stdin = std::io::stdin();
    let mut stdout = std::io::stdout();
    let mut buf = String::new();

    loop {
        let prompt = debugger.format_prompt();
        let line = match r8db::prompt_for_input(&prompt, &mut buf, &mut stdin.lock(), &mut stdout)?
        {
            ControlFlow::Continue(line) => line,
            ControlFlow::Break(()) => {
                println!();
                break;
            }
        };

        let instruction = match r8db::Instruction::parse(line) {
            Ok(instruction) => instruction,
            Err(err) => {
                eprintln!("{err}");
                continue;
            }
        };

        debugger.run_instruction(instruction);
    }

    Ok(())
}
