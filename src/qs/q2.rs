pub fn exec_intcode(mut ops: Vec<usize>) -> Result<usize, &'static str>{
    let mut i = 0_usize;
    loop {
        let instruction_size:usize;
        let opcode = ops.get(i).ok_or("Invalid intcode - tried to access out of range address!")?;
        match opcode {
            99 => return Ok(ops[0]),
            1 => {
                instruction_size = 4;
                let loc1 = *ops.get(i+1).ok_or("Invalid intcode - tried to access out of range address!")?;
                let loc2 = *ops.get(i+2).ok_or("Invalid intcode - tried to access out of range address!")?;
                let loc3 = *ops.get(i+3).ok_or("Invalid intcode - tried to access out of range address!")?;
                if ops.get(loc3).is_none() { return Err("Invalid intcode - tried to access out of range address!") }
                let par1 = ops.get(loc1).ok_or("Invalid intcode - tried to access out of range address!")?;
                let par2 = ops.get(loc2).ok_or("Invalid intcode - tried to access out of range address!")?;
                ops[loc3] = par1 + par2;
            },
            2 => {
                instruction_size = 4;
                let loc1 = *ops.get(i+1).ok_or("Invalid intcode - tried to access out of range address!")?;
                let loc2 = *ops.get(i+2).ok_or("Invalid intcode - tried to access out of range address!")?;
                let loc3 = *ops.get(i+3).ok_or("Invalid intcode - tried to access out of range address!")?;
                let addr = *ops.get(loc3).ok_or("Invalid intcode - tried to access out of range address!")?;
                let par1 = ops.get(loc1).ok_or("Invalid intcode - tried to access out of range address!")?;
                let par2 = ops.get(loc2).ok_or("Invalid intcode - tried to access out of range address!")?;
                ops[addr] = par1 * par2;
            }
            _ => return Err("Invalid opcode encountered!")
        };
        i += instruction_size;

    }
}

pub fn q2(mut intcode: Vec<usize>, noun: usize, verb: usize) -> Result<usize, &'static str>{
    intcode[1] = noun;
    intcode[2] = verb;
    exec_intcode(intcode)
}
