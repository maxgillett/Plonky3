// use crate::Challenger;
// use alloc::vec;
// use alloc::vec::Vec;
// use core::marker::PhantomData;
// use p3_field::field::Field;
// use p3_symmetric::hasher::{CryptographicHasher, VecToArrHasher};
//
// pub struct HashChallenger<F: Field, H: VecToArrHasher<F, OUT_LEN>, const OUT_LEN: usize> {
//     input_buffer: Vec<F>,
//     output_buffer: Vec<F>,
//     _phantom_f: PhantomData<F>,
//     _phantom_h: PhantomData<H>,
// }
//
// impl<F: Field, H: VecToArrHasher<F, OUT_LEN>, const OUT_LEN: usize>
//     HashChallenger<F, H, OUT_LEN>
// {
//     pub fn new(initial_state: Vec<F>) -> Self {
//         Self {
//             input_buffer: initial_state,
//             output_buffer: vec![],
//             _phantom_f: PhantomData,
//             _phantom_h: PhantomData,
//         }
//     }
//
//     fn flush(&mut self) {
//         let inputs = self.input_buffer.drain(..).collect();
//         let output = H::hash(inputs);
//
//         self.output_buffer = output.to_vec();
//
//         // Chaining values.
//         self.input_buffer.extend(output.to_vec());
//     }
// }
//
// impl<F: Field, H: VecToArrHasher<F, OUT_LEN>, const OUT_LEN: usize> Challenger<F>
//     for HashChallenger<F, H, OUT_LEN>
// {
//     fn observe_element(&mut self, element: F) {
//         // Any buffered output is now invalid.
//         self.output_buffer.clear();
//
//         self.input_buffer.push(element);
//     }
//
//     fn random_element(&mut self) -> F {
//         if self.output_buffer.is_empty() {
//             self.flush()
//         }
//         self.output_buffer
//             .pop()
//             .expect("Output buffer should be non-empty")
//     }
// }
