use year2019::intcode::Program;

fn main() {
    let mut program = Program::from_stdin();
    program.save();
    program.set_stdin(vec![1]);
    program.run();
    println!("{}", program.stdout());

    program.reset();
    program.set_stdin(vec![2]);
    program.run();
    println!("{}", program.stdout());
}
