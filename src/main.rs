use miden_lib::transaction::TransactionKernel;
use miden_vm::{DefaultHost, StackInputs, AdviceInputs, execute, execute_iter, Assembler};
use miden_processor::ExecutionOptions;

fn main() -> () {


    // Instantiate the assembler with multiple options at once
    let assembler = TransactionKernel::assembler()
        .with_debug_mode(true)
        .with_warnings_as_errors(true);

    // let assembler = Assembler::default()
    //     .with_debug_mode(true)
    //     .with_warnings_as_errors(true);

    // use an empty list as initial stack
    let stack_inputs = StackInputs::default();
    // do not include any initial advice data
    let advice_inputs = AdviceInputs::default();
    // instantiate a default host (with an empty advice provider)
    let mut host = DefaultHost::default();
    // instantiate default execution options
    let exec_options = ExecutionOptions::default();

    // Assemble our program
    let program = assembler.assemble_program("
   
    begin
        push.1.2
        debug.stack
        drop drop
    end
    ").unwrap();

    // let trace = execute(&program, stack_inputs.clone(), advice_inputs.clone(), &mut host, exec_options).unwrap();

    for vm_state in execute_iter(
        &program,
        stack_inputs,
        // advice_inputs,
        &mut host,
    ) {
        match vm_state {
            Ok(vm_state) => println!("{:?}", vm_state),
            Err(_) => println!("something went terribly wrong!"),
        }
    }

    // println!("Program hash: {:?}", program.hash());

}

