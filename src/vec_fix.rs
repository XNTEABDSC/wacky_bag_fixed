
use crate::num::Num;
//pub struct Vec2F(pub Num,pub Num);


pub type VecFix<const DIM:usize>=nalgebra::SVector<Num,DIM>;

pub type VecFix2=VecFix<2>;

// #[inline]
// pub fn dist_sq<const DIM:usize>(a: VecFix<DIM>)->Num {
//     a*a
// }

// #[inline]
// pub fn dist<const DIM:usize>(a: VecFix<DIM>)->Num {
//     cordic::sqrt(dist_sq(a))
// }
// pub fn norm<const DIM:usize>(a:VecFix<DIM>)->VecFix<DIM> {
//     a/dist(a)
// }
// /*
// pub fn dir<const N:usize>(a:VecFix<2>)->[Num;N] {
//     array::from_fn(|i|{
//         let i2= if (i+1)<N{i+1}else{0};
//         cordic::atan2(a[i2], a[i])
//     })
// }
// pub fn from_dir<const N:usize>(dir:[Num;N])->VecFix<N>{
//     let mut cur_len=Num::ONE;
//     VecFix::new(array::from_fn(|i|{
//         let len=cur_len*cordic::cos(dir[i]);
//         cur_len=len;
//         len
//     }))
// } */

// #[inline]
// pub fn dir_2(a: VecFix2)->Num {
//     cordic::atan2(a[1], a[0])
// }

// #[inline]
// pub fn from_rot_2(rot:Num)->VecFix2{
//     VecFix2::new([cos(rot),sin(rot)])
// }
// #[inline]
// pub fn from_polar_2(rot:Num,dist:Num)->VecFix2 {
//     VecFix2::new([cos(rot)*dist,sin(rot)*dist])
// }