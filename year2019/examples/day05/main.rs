use year2019::intcode::Program;

fn main() {
    let mut program = Program::new();
    program.set_stdin(vec![1]);
    program.run();
    println!("{:?}", program.get_stdout().iter().last().expect("program produced empty output"));
}
