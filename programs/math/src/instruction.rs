use solana_program::{
    program_error::ProgramError,
};

const I64_BYTES: usize = 8;

#[derive(Debug)]
pub enum MathInstruction{
    Add{
        num: i64,
    },
    Subtract{
        num: i64,
    },
    Multiply{
        num: i64,
    }
}

impl MathInstruction{
    pub fn unpack(input: &[u8]) -> Result<MathInstruction,ProgramError>{
        let (tag,rest) = input.split_first().ok_or(ProgramError::InvalidArgument)?;

        let num = rest.get(..I64_BYTES)
        .and_then(|slice|slice.try_into().ok())
        .map(i64::from_le_bytes)
        .ok_or(ProgramError::InvalidArgument)?;

        return match *tag {
            0 => {

                Ok(MathInstruction::Add{num : num})
            },
            1 => {
                

                Ok(MathInstruction::Subtract{
                    num : num,
                })
            },
            2 => {

                Ok(MathInstruction::Multiply{
                    num: num
                })
            },
            _ => Err(ProgramError::InvalidArgument) 
        };
    }
}

#[cfg(test)]
mod tests{



    use super::*;

    #[test]
    fn unpack(){
        let add_tag = 0_u8;
        let subtract_tag = 1_u8;
        let multiply_tag = 2_u8;
        let invalid_tag = 3_u8;
        let num = 45_i64.to_le_bytes();
        {
            let mut input = vec![add_tag];
            input.extend(num.iter());

            let output = MathInstruction::unpack(&input);
            assert!(output.is_ok());
            match output.unwrap() {
                MathInstruction::Add{num} =>{
                    assert_eq!(num,45);
                }
                _ => panic!()
            };
        }

        {
            let mut input = vec![subtract_tag];
            input.extend(num.iter());

            let output = MathInstruction::unpack(&input);
            assert!(output.is_ok());
            match output.unwrap(){
                MathInstruction::Subtract{num} =>assert_eq!(num,45),
                _ => panic!()
            }
        }

        {
            let mut input = vec![multiply_tag];
            input.extend(num.iter());
            
            let output = MathInstruction::unpack(&input);
            assert!(output.is_ok());
            match output.unwrap(){
                MathInstruction::Multiply{num  }=> assert_eq!(num,45),
                _ => panic!()
            };
        }

        {
            let mut input = vec![invalid_tag];
            input.extend(num.iter());

            let output = MathInstruction::unpack(&input);
            assert!(output.is_err());
        }
    }
}