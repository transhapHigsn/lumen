use super::*;

use std::sync::{Arc, RwLock};

use num_traits::Num;

use crate::environment::{self, Environment};
use crate::process::IntoProcess;

#[test]
fn with_atom_errors_badarg() {
    let environment_rw_lock: Arc<RwLock<Environment>> = Default::default();
    let process_rw_lock = environment::process(Arc::clone(&environment_rw_lock));
    let mut process = process_rw_lock.write().unwrap();
    let tuple = Term::str_to_atom("atom", Existence::DoNotCare, &mut process).unwrap();

    assert_bad_argument!(erlang::tuple_size_1(tuple, &mut process), &mut process);
}

#[test]
fn with_local_reference_errors_badarg() {
    let environment_rw_lock: Arc<RwLock<Environment>> = Default::default();
    let process_rw_lock = environment::process(Arc::clone(&environment_rw_lock));
    let mut process = process_rw_lock.write().unwrap();
    let tuple = Term::local_reference(&mut process);

    assert_bad_argument!(erlang::tuple_size_1(tuple, &mut process), &mut process);
}

#[test]
fn with_empty_list_errors_badarg() {
    let environment_rw_lock: Arc<RwLock<Environment>> = Default::default();
    let process_rw_lock = environment::process(Arc::clone(&environment_rw_lock));
    let mut process = process_rw_lock.write().unwrap();
    let tuple = Term::EMPTY_LIST;

    assert_bad_argument!(erlang::tuple_size_1(tuple, &mut process), &mut process);
}

#[test]
fn with_list_errors_badarg() {
    let environment_rw_lock: Arc<RwLock<Environment>> = Default::default();
    let process_rw_lock = environment::process(Arc::clone(&environment_rw_lock));
    let mut process = process_rw_lock.write().unwrap();
    let tuple = list_term(&mut process);

    assert_bad_argument!(erlang::tuple_size_1(tuple, &mut process), &mut process);
}

#[test]
fn with_small_integer_errors_badarg() {
    let environment_rw_lock: Arc<RwLock<Environment>> = Default::default();
    let process_rw_lock = environment::process(Arc::clone(&environment_rw_lock));
    let mut process = process_rw_lock.write().unwrap();
    let tuple = 0usize.into_process(&mut process);

    assert_bad_argument!(erlang::tuple_size_1(tuple, &mut process), &mut process);
}

#[test]
fn with_big_integer_errors_badarg() {
    let environment_rw_lock: Arc<RwLock<Environment>> = Default::default();
    let process_rw_lock = environment::process(Arc::clone(&environment_rw_lock));
    let mut process = process_rw_lock.write().unwrap();
    let big_integer_term = <BigInt as Num>::from_str_radix("576460752303423489", 10)
        .unwrap()
        .into_process(&mut process);

    assert_bad_argument!(
        erlang::tuple_size_1(big_integer_term, &mut process),
        &mut process
    );
}

#[test]
fn with_float_errors_badarg() {
    let environment_rw_lock: Arc<RwLock<Environment>> = Default::default();
    let process_rw_lock = environment::process(Arc::clone(&environment_rw_lock));
    let mut process = process_rw_lock.write().unwrap();
    let tuple = 1.0.into_process(&mut process);

    assert_bad_argument!(erlang::tuple_size_1(tuple, &mut process), &mut process);
}

#[test]
fn with_local_pid_errors_badarg() {
    let environment_rw_lock: Arc<RwLock<Environment>> = Default::default();
    let process_rw_lock = environment::process(Arc::clone(&environment_rw_lock));
    let mut process = process_rw_lock.write().unwrap();
    let tuple = Term::local_pid(0, 0, &mut process).unwrap();

    assert_bad_argument!(erlang::tuple_size_1(tuple, &mut process), &mut process);
}

#[test]
fn with_external_pid_errors_badarg() {
    let environment_rw_lock: Arc<RwLock<Environment>> = Default::default();
    let process_rw_lock = environment::process(Arc::clone(&environment_rw_lock));
    let mut process = process_rw_lock.write().unwrap();
    let tuple = Term::external_pid(1, 0, 0, &mut process).unwrap();

    assert_bad_argument!(erlang::tuple_size_1(tuple, &mut process), &mut process);
}

#[test]
fn with_tuple_without_elements_is_zero() {
    let environment_rw_lock: Arc<RwLock<Environment>> = Default::default();
    let process_rw_lock = environment::process(Arc::clone(&environment_rw_lock));
    let mut process = process_rw_lock.write().unwrap();
    let empty_tuple = Term::slice_to_tuple(&[], &mut process);
    let zero_term = 0usize.into_process(&mut process);

    assert_eq_in_process!(
        erlang::tuple_size_1(empty_tuple, &mut process),
        Ok(zero_term),
        process
    );
}

#[test]
fn with_tuple_with_elements_is_element_count() {
    let environment_rw_lock: Arc<RwLock<Environment>> = Default::default();
    let process_rw_lock = environment::process(Arc::clone(&environment_rw_lock));
    let mut process = process_rw_lock.write().unwrap();
    let element_vec: Vec<Term> = (0..=2usize).map(|i| i.into_process(&mut process)).collect();
    let element_slice: &[Term] = element_vec.as_slice();
    let tuple = Term::slice_to_tuple(element_slice, &mut process);
    let arity_term = 3usize.into_process(&mut process);

    assert_eq_in_process!(
        erlang::tuple_size_1(tuple, &mut process),
        Ok(arity_term),
        process
    );
}

#[test]
fn with_map_errors_badarg() {
    let environment_rw_lock: Arc<RwLock<Environment>> = Default::default();
    let process_rw_lock = environment::process(Arc::clone(&environment_rw_lock));
    let mut process = process_rw_lock.write().unwrap();
    let tuple = Term::slice_to_map(&[], &mut process);

    assert_bad_argument!(erlang::tuple_size_1(tuple, &mut process), &mut process);
}

#[test]
fn with_heap_binary_errors_badarg() {
    let environment_rw_lock: Arc<RwLock<Environment>> = Default::default();
    let process_rw_lock = environment::process(Arc::clone(&environment_rw_lock));
    let mut process = process_rw_lock.write().unwrap();
    let tuple = Term::slice_to_binary(&[0, 1, 2], &mut process);

    assert_bad_argument!(erlang::tuple_size_1(tuple, &mut process), &mut process);
}

#[test]
fn with_subbinary_errors_badarg() {
    let environment_rw_lock: Arc<RwLock<Environment>> = Default::default();
    let process_rw_lock = environment::process(Arc::clone(&environment_rw_lock));
    let mut process = process_rw_lock.write().unwrap();
    let binary_term =
        Term::slice_to_binary(&[0b0000_00001, 0b1111_1110, 0b1010_1011], &mut process);
    let tuple = Term::subbinary(binary_term, 0, 7, 2, 1, &mut process);

    assert_bad_argument!(erlang::tuple_size_1(tuple, &mut process), &mut process);
}
