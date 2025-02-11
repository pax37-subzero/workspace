// src/constants.rs

pub const RAYDIUM_V4: &str = "675kPX9MHTjS2zt1qfr1NYHuzeLXfQM9H24wFSUt1Mp8";
pub const RAYDIUM_CL: &str = "27haf8L6oxUeXrHrgEgsexjSY5hbVUWEmvv9Nyxg8vQv";
pub const JUPITER_V6: &str = "JUP6LkbZbjS1jKKwapdHNy74zcZ3tLUZoi5QNyVTaV4";

pub fn get_dex_programs() -> Vec<&'static str> {
    vec![RAYDIUM_V4, RAYDIUM_CL, JUPITER_V6]
}

pub fn is_supported_dex(outer_program: &str, inner_program: &str) -> bool {
    let programs = get_dex_programs();
    programs.contains(&outer_program) || programs.contains(&inner_program)
}