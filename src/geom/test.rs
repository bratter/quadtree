
// #[derive(Clone, Copy)]
// pub struct First<T: Copy> {
//     x: T,
// }

// impl <T> First<T> {
//     pub fn go(&self) -> i32 {
//         42
//     }
// }

// pub struct Second<T> {
//     y: First<T>,
//     z: T,
// }

// impl <T> Second<T> {
//     pub fn go(&self) -> i32 {
//         let first = self.y;
//         first.go()
//     }
// }