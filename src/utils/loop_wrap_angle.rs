use crate::num::Num;

pub fn loop_wrap_angle_assign(angle:&mut Num){
    wacky_bag::utils::loop_wrap::loop_wrap_assign(angle, -Num::PI, Num::PI, &(Num::PI*2));
}

pub fn loop_wrap_angle(angle:Num)->Num{
    wacky_bag::utils::loop_wrap::loop_wrap(angle, -Num::PI, Num::PI, &(Num::PI*2))
}