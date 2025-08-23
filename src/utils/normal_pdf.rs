use crate::num::Num;

pub fn normal_pdf(x:Num)->Num{
    return Num::FRAC_1_SQRT_2PI * cordic::exp(-x/2*x)
}