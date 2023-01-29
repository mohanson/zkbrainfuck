use zkbrainfuck::code;
use zkbrainfuck::interpreter::Interpreter;

#[test]
fn test_run_hello_world() {
    let program = code::compile(include_bytes!("../res/hello_world.bf").to_vec());
    let mut vm = Interpreter::new();
    vm.set_code(program);
    vm.run();
    assert_eq!(vm.matrix.output_matrix, code::easygen("Hello World!\n"));
}

#[test]
fn test_run_neptune() {
    let program = code::compile(include_bytes!("../res/neptune_tutorial.bf").to_vec());
    let mut vm = Interpreter::new();
    vm.set_code(program);
    vm.set_input(code::easygen("a"));
    vm.run();
    assert_eq!(vm.matrix.input_matrix, code::easygen("a"));
    assert_eq!(vm.matrix.output_matrix, code::easygen("bc"));
}
