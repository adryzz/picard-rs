#![allow(non_camel_case_types)]


use buffer::ShaderBuffer;
use dvle::Uniform;
use smallvec::SmallVec;
use thiserror::Error;
use dvle::DvleData;
use dvle::StackEntry;
use dvle::MAX_UNIFORM;

pub mod alloc;
pub mod stuff;
pub mod buffer;
pub mod dvle;
mod utils;
mod maestro;
const MAX_OPDESC: usize = 128;
const MAX_STACK: usize = 32;


pub struct ShaderAssembler {
    g_output_buf: Vec<u32>,
    g_total_dvle_count: u32,
    g_opdesc_table: SmallVec<[u32; MAX_OPDESC]>,
    g_opdesc_masks: SmallVec<[u32; MAX_OPDESC]>,
    dvle_table_iter: Vec<DvleData>,
    g_stack: SmallVec<[StackEntry; MAX_STACK]>,
    g_stack_pos: usize,
    g_uniform_table: SmallVec<[Uniform; MAX_UNIFORM]>

}

impl ShaderAssembler {
    pub fn new() -> Self {
        Self {
            g_output_buf: Vec::new(),
            g_total_dvle_count: 0,
            g_opdesc_table: SmallVec::new(),
            g_opdesc_masks: SmallVec::new(),
            dvle_table_iter: Vec::new(),
            g_stack: SmallVec::new(),
            g_stack_pos: 0,
            g_uniform_table: SmallVec::new()
        }
    }

    /// Assembles a shader into a ShaderBuffer
    pub fn assemble_shader(&mut self, shader: &str) -> Result<ShaderBuffer, PicassoError> {
        let mut buf = ShaderBuffer::new();

        let prog_size = self.g_output_buf.len() as u32;
        let dvlp_size = 10*4 + prog_size*4 + self.g_opdesc_table.len() as u32 *8;

        // write DVLB header
        buf.write_u32(0x424C5644)?; // DVLB
        buf.write_u32(self.g_total_dvle_count)?; // Number of DVLEs

        // Calculate and write DVLE offsets
        let mut cur_off = 2*4 + self.g_total_dvle_count*4 + dvlp_size;
        for dvle in &self.dvle_table_iter {
            if dvle.no_dvle {
                continue;
            }

            buf.write_u32(cur_off)?;
            cur_off += 16*4;
            cur_off += (dvle.constant_table.len()*20) as u32;
            cur_off += (dvle.constant_table.len()*8) as u32;
            cur_off += (dvle.uniform_table.len()*8) as u32;
            cur_off += dvle.symbol_size as u32;
            cur_off  = (cur_off + 3) &! 3; // Word alignment
        }

        // Write DVLP header
        buf.write_u32(0x504C5644)?; // DVLP
        buf.write_u32(0)?; // version
        buf.write_u32(10*4)?; // offset to shader binary blob
        buf.write_u32(prog_size)?; // size of shader binary blob
        buf.write_u32(10*4 + prog_size*4)?; // offset to opdesc table
        buf.write_u32(self.g_opdesc_table.len() as u32)?; // number of opdescs
        buf.write_u32(dvlp_size)?; // offset to symtable (TODO)
        buf.write_u32(0)?; // ????
        buf.write_u32(0)?; // ????
        buf.write_u32(0)?; // ????

        // Write program
        for it in &self.g_output_buf {
            buf.write_u32(*it)?; // TODO: CHECK THIS FOR CORRECTNESS
        }

        for i in 0..self.g_opdesc_table.len() { // ??? this is writing a DWORD (u64) but the array is of WORD (u32)
            buf.write_u64(self.g_opdesc_table[i] as u64)?; 
        }

        for dvle in &self.dvle_table_iter {
            buf.write_u32(0x454C5644)?; // DVLE
            buf.write_u16(0x1002)?; // maybe version?
            buf.write_u8(if dvle.is_geo_shader {1} else {0})?; // Shader type
            buf.write_u8(if dvle.is_merge {1} else {0})?;
            buf.write_u32(dvle.entry_start as u32)?; // offset to main
            buf.write_u32(dvle.entry_end as u32)?; // offset to end of main
            buf.write_u16(dvle.input_mask)?;
            buf.write_u16(dvle.output_mask)?;
            buf.write_u8(dvle.geo_shader_type)?;
            buf.write_u8(dvle.geo_shader_fixed_start)?;
            buf.write_u8(dvle.geo_shader_variable_num)?;
            buf.write_u8(dvle.geo_shader_fixed_num)?;
            buf.write_u32(cur_off)?; // offset to constant table
            buf.write_u32(dvle.constant_table.len() as u32)?; // size of constant table
            cur_off += dvle.constant_table.len() as u32 *5*4;
            buf.write_u32(cur_off)?; // offset to label table (TODO)
            buf.write_u32(0)?; // size of label table (TODO)
            buf.write_u32(cur_off)?; // offset to output table
            buf.write_u32(dvle.output_table.len() as u32)?; // size of output table
            cur_off += dvle.output_table.len() as u32 *8;
            buf.write_u32(cur_off)?; // offset to uniform table
            buf.write_u32(dvle.uniform_table.len() as u32)?; // size of uniform table
            cur_off += dvle.uniform_table.len() as u32*8;
            buf.write_u32(cur_off)?; // offset to symbol table
            buf.write_u32(dvle.symbol_size as u32)?; // size of symbol table

            // Sort uniforms by position
            //dvle.u

            //TODO: CONTINUE
        }
        
        Ok(buf)
    }

    /// Assembles multiple shaders into a ShaderBuffer
    pub fn assemble_shaders(&mut self, shaders: Vec<&str>) -> Result<ShaderBuffer, PicassoError> {
        Err(PicassoError::UnknownError)
    }
}


#[derive(Debug, Error)]
pub enum PicassoError {
    /// Remove me when the port is done
    #[error("Unknown Error")]
    UnknownError,

    #[error("I/O Error")]
    IoError(#[from] std::io::Error),

    #[error("Missing parameter")]
    MissingParam,

    #[error("Invalid instruction")]
    InvalidInstruction,

    #[error("Instruction outside block")]
    InstructionOutsideBlock,
}