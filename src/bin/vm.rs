use rustsimplevm::vm::Machine;

pub fn main() -> Result<(), String>{
    let mut vm = Machine::new();
    vm.step()?;
    vm.step()
}