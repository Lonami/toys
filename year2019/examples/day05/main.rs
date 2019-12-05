use year2019::intcode::Program;

fn main() {
    let mut program = Program::new();
    program.save();
    program.set_stdin(vec![1]);
    program.run();
    println!("{}", program.get_stdout().iter().last().expect("program produced empty output"));

    program.reset();
    program.set_stdin(vec![5]);
    program.run();
    println!("{}", program.get_stdout().iter().next().expect("program produced empty output"));
}
