use std::collections::HashMap;

/// Emulator of the **CARDIAC** cardboard computer.  
/// info in [wikipedia](wikipedia.org/wiki/CARDboard_Illustrative_Aid_to_Computation)
pub struct Assembler {
    memory: [u32; 100],
    accumulator: i32,
    target: u32,
    flag: bool,
    step: i32,
    input_deck: Vec<i32>,
    output_deck: Vec<i32>,
    instruction_map: HashMap<u32, fn(&Self, usize)>,
}

impl Assembler {
    /// Creates an new `Assembler`
    pub fn new() -> Self {
        let mut instruction_map: HashMap<u32, fn(&Self, usize)> = HashMap::new();
        instruction_map.insert(0, Self::inp);
        instruction_map.insert(0, Self::cla);
        instruction_map.insert(0, Self::add);
        instruction_map.insert(0, Self::tac);
        instruction_map.insert(0, Self::sft);
        instruction_map.insert(0, Self::out);
        instruction_map.insert(0, Self::sto);
        instruction_map.insert(0, Self::sub);
        instruction_map.insert(0, Self::jmp);
        instruction_map.insert(0, Self::hrs);

        let mut memory: [u32; 100] = [0; 100];
        memory[0] = 1;
        memory[99] = 800;

        Self {
            memory: memory,
            accumulator: 0,
            target: 0,
            flag: true,
            step: 0,
            input_deck: Vec::new(),
            output_deck: Vec::new(),
            instruction_map: instruction_map,
        }
    }

    fn inp(&self, address: usize) {
        println!("inp {address}")
    }

    fn cla(&self, address: usize) {
        println!("inp {address}")
    }

    fn add(&self, address: usize) {
        println!("inp {address}")
    }

    fn tac(&self, address: usize) {
        println!("inp {address}")
    }

    fn sft(&self, address: usize) {
        println!("inp {address}")
    }

    fn out(&self, address: usize) {
        println!("inp {address}")
    }

    fn sto(&self, address: usize) {
        println!("inp {address}")
    }

    fn sub(&self, address: usize) {
        println!("inp {address}")
    }

    fn jmp(&self, address: usize) {
        println!("inp {address}")
    }

    fn hrs(&self, address: usize) {
        println!("inp {address}")
    }

    pub fn reset(&mut self) {
        self.target = 0;
        self.step = 0;
        self.accumulator = 0;
        self.input_deck = Vec::new();
        self.output_deck = Vec::new();
        self.flag = true;
    }

    pub fn clear_memory(&mut self) {
        self.memory = [0; 100];
        self.memory[0] = 1;
        self.memory[99] = 800;
    }

    pub fn update_cell(&mut self, address: u32, instruction: u32) {
        self.memory[address as usize] = instruction;
    }

    pub fn load_program(&mut self, program: HashMap<u32, u32>) {
        for (address, instruction) in program {
            self.memory[address as usize] = instruction;
            // println!("Address {address}; Instruction {instruction};")
        } 
    }

    pub fn next_step(&mut self) {
        let instruction: u32 = self.memory[self.target as usize];

        let opcode: u32 = instruction / 100;
        let address: u32 = instruction % 100;
        
        if let Some(instruction_fn) = self.instruction_map.get(&opcode) {
            instruction_fn(self, address as usize);
            self.step = self.step + 1;
        } else {
            panic!("Opcode undefined: {}", opcode);
        }
    }

}