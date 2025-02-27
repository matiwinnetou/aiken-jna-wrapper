extern crate core;

mod transaction;
mod build;

use std::os::raw::c_char;
use std::ffi::{CStr, CString};
use std::{mem, panic};

#[no_mangle]
#[repr(C)]
#[allow(missing_copy_implementations)]
#[derive(Clone)]
pub struct SlotConfig {
    pub slot_length: u32,
    pub zero_slot: u64,
    pub zero_time: u64,
}

#[no_mangle]
#[repr(C)]
#[allow(missing_copy_implementations)]
#[derive(Clone)]
pub struct InitialBudget {
    pub mem: i64,
    pub cpu: i64,
}

#[no_mangle]
#[allow(non_snake_case)]
pub fn eval_phase_two(tx_hex: *const c_char,
                      inputs: *const c_char,
                      outputs: *const c_char,
                      cost_mdls: *const c_char,
                      initial_budget: InitialBudget,
                      slot_config: SlotConfig) -> *const c_char {
    let result = panic::catch_unwind(|| {
        let tx_hex = to_string(tx_hex);
        let inputs = to_string(inputs);
        let outputs = to_string(outputs);
        let cost_mdls = to_string(cost_mdls);

        let ak_ex_budget = uplc::machine::cost_model::ExBudget {
            mem: initial_budget.mem,
            cpu: initial_budget.cpu,
        };

        let ak_slot_config = uplc::tx::script_context::SlotConfig {
            zero_time: slot_config.zero_time,
            zero_slot: slot_config.zero_slot,
            slot_length: slot_config.slot_length,
        };

        let result = transaction::eval_phase_two(&tx_hex, &inputs, &outputs, &cost_mdls, ak_ex_budget, ak_slot_config);
        match result {
            Ok(redeemer) => {
                to_ptr(redeemer)
            }
            Err(err) => {
                to_ptr(format!("{:?}", err))
            }
        }
    });

    match result {
        Ok(c) => c,
        Err(_cause) => {
            to_ptr(String::new())
        }
    }
}

/// Convert a native string to a Rust string
fn to_string(pointer: *const c_char) -> String {
    let c_str: &CStr = unsafe { CStr::from_ptr(pointer) };
    c_str.to_str().unwrap().to_string()
}

/// Convert a Rust string to a native string
fn to_ptr(string: String) -> *const c_char {
    let cs = CString::new(string.as_bytes()).unwrap();
    let ptr = cs.as_ptr();
    // Tell Rust not to clean up the string while we still have a pointer to it.
    // Otherwise, we'll get a segfault.
    mem::forget(cs);
    ptr
}

#[no_mangle]
#[allow(non_snake_case)]
fn dropCharPointer(pointer: *const c_char) {
    unsafe {
        mem::drop(pointer);
    }
}

#[no_mangle]
#[allow(non_snake_case)]
fn printPointer(pointer: *const c_char) {
    println!("Print pointer >>> {}", to_string(pointer));
}

#[cfg(test)]
mod tests {}
