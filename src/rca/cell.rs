
use std::io::Error;

#[allow(unused)]
use sysinfo::System;

#[allow(unused)]
use std::time::SystemTime;

#[allow(unused)]
use std::fmt::write;

/* Project Dependencies */
use crate::rca::{ 
    DataPlane, 
};
use crate::signals::samples::*;
use crate::signals::quantization::*;


/*******************************************************************************
 * (1) Cell Data 
******************************************************************************/

/* Cells 
 * Description: Each cell can get access to the system context or data, but it cannot modify the context. Only the engine has authority to modify state. 
 * Nature: Each cell HAS-A task
 */

/* Status: MUTABLE */
#[derive(Debug, PartialEq, Clone)]
pub enum CellData {
    None,
    Byte(u8), 
    // Add cell data types here
    F64(f64),
    I64(i64),
}

impl Default for CellData {
    fn default() -> Self {
        CellData::None
    }
}

/* Status: FREEZE */
#[derive(Debug)]
pub struct Cell {
    pub id: usize,
    pub task: Task,
}

/* Status: FREEZE */
impl PartialEq for Cell {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }

}

/* Status: FREEZE */
impl Cell {
    pub fn default() -> [Self; CELLS] {
        let tasks: [Self; CELLS] = core::array::from_fn(|i| Cell {
            id: i,
            task: Task::Default,
        });
        tasks
    }

    pub fn execute(&mut self, context: &DataPlane, handoff: CellData) -> Result<CellData, Error> {
       self.task.access_task(context, handoff) 
    }
}

/*******************************************************************************
 * (2) Tasks 
******************************************************************************/

/* Tasks 
 * Description: Tasks help formulate cells. 
 * Nature: Each task HAS-A type and operation/behavior.
 */

/* Status: MUTABLE */
#[allow(unused)]
pub const CELLS: usize = 2;

/* Status: MUTABLE */
#[derive(Debug)]
pub enum Task {
    Default,
    GetSinSample,
    TakeQuantizedSample,
    // Add tasks here
}

/* Status: MUTABLE */
impl Task {
    pub fn access_task(&self, _ctx: &DataPlane, _handoff: CellData) ->  Result<CellData, Error> {
        match self {

            Task::Default => {
                let transform = CellData::Byte(0x2A); // 42
                Ok(transform)
            }

            Task::GetSinSample => {
                // Ok(CellData::F64(give_simulated_sample(1.0, 200.0)))
                Ok(CellData::F64(give_simulated_sample(0.0, 0.0)))
            }

            Task::TakeQuantizedSample => {
                match _handoff {
                    CellData::F64(sample) => {
                        let sample = CellData::I64(quantize_sample(sample, 32));
                        Ok(sample)
                    }

                    _ => Ok(CellData::None)
                }
            }

            // Add task procedures here

        }

    }
}