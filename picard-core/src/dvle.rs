use smallvec::SmallVec;

use crate::utils::bit;

pub const MAX_UNIFORM: usize = 0x60;
pub const MAX_CONSTANT: usize = 0x60;
pub const MAX_OUTPUT: usize = 16;

#[derive(Debug, Clone)]
pub struct Uniform {
    name: String,
    pos: u32,
    size: u32,
    r#type: u32
}

impl Uniform {
    pub fn new() -> Self {
        Self {
            name: String::new(),
            pos: 0,
            size: 0,
            r#type: 0
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Constant {
    reg_id: u32,
    r#type: u32,
    param: Param
}

#[derive(Debug, Clone, Copy)]
pub enum Param {
    FParam([f32; 4]),
    IParam([u8; 4]),
    BParam(bool)
}

#[derive(Debug, Clone)]
pub struct StackEntry {
    r#type: usize,
    pos: usize,
    extra: StackExtra,
}

#[derive(Debug, Clone)]
pub enum StackExtra {
    StrExtra(String),
    UIntExtra(usize),
}

#[derive(Debug, Clone)]
pub struct DvleData {
    pub file_name: String,
    pub entrypoint: String,

    pub entry_start: usize,
    pub entry_end: usize,

    pub no_dvle: bool,
    pub is_geo_shader: bool,
    pub is_compat_geo_shader: bool,
    pub is_merge: bool,

    pub input_mask: u16,
    pub output_mask: u16,

    pub geo_shader_type: u8,
    pub geo_shader_fixed_start: u8,
    pub geo_shader_variable_num: u8,
    pub geo_shader_fixed_num: u8,

    pub uniform_table: SmallVec<[Uniform; MAX_UNIFORM]>,
    pub symbol_size: usize,

    pub constant_table: SmallVec<[Constant; MAX_CONSTANT]>,

    pub output_table: SmallVec<[u32; MAX_CONSTANT]>,
    pub output_used_reg: u32,
}

impl DvleData {
    pub fn new(file_name: String) -> Self {
        Self {
            file_name,
            entrypoint: "main".to_string(),

            no_dvle: false,
            is_geo_shader: false,
            is_compat_geo_shader: false,
            is_merge: false,

            input_mask: 0,
            output_mask: 0,
            geo_shader_type: 0,
            geo_shader_fixed_start: 0,
            geo_shader_variable_num: 0,
            geo_shader_fixed_num: 0,

            symbol_size: 0,
            output_used_reg: 0,

            entry_start: 0,
            entry_end: 0, // FIXME: fix this shitty ass constructor
            uniform_table: SmallVec::new(),
            constant_table: SmallVec::new(),
            output_table: SmallVec::new(),
        }
    }

    pub fn max_output_reg(&self) -> u32 {
        return if self.is_geo_shader {0x07} else {0x10}
    }

    pub fn find_free_input(&self) -> i32 {
        for i in 0..16 {
            if (self.input_mask & bit(i)) == 0 {
                return i.into();
            }

            return -1; // TODO:  try replacing this with a Result
        }


        return -1;
    }

    pub fn find_free_output(&self) -> i32 {
        for i in 0..self.max_output_reg() {
            if (self.output_mask & bit(i) as u16) == 0 {
                return i as i32;
            }
        }

        return -1; // TODO:  try replacing this with a Result
    }

    pub fn uses_gsh_space(&self) -> bool {
        self.is_geo_shader && !self.is_compat_geo_shader
    }
}