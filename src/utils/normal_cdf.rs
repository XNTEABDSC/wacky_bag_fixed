use crate::num::Num;



pub fn normal_cdf(x:Num)->Num {
    const A1: Result<Num, fixed::ParseFixedError> = Num::from_str("0.254829592");
    const A2: Result<Num, fixed::ParseFixedError> = Num::from_str("-0.284496736");
    const A3: Result<Num, fixed::ParseFixedError> = Num::from_str("1.421413741");
    const A4: Result<Num, fixed::ParseFixedError> = Num::from_str("-1.453152027");
    const A5: Result<Num, fixed::ParseFixedError> = Num::from_str("1.061405429");
    const P: Result<Num, fixed::ParseFixedError> = Num::from_str("0.3275911");
    let sign = Num::signum(x);
    let x2=x.abs()*Num::FRAC_1_SQRT_2;
    let t= Num::ONE/(Num::ONE+P.unwrap()*x2);
    let y=Num::ONE-(((((A5.unwrap()*t + A4.unwrap())*t) + A3.unwrap())*t + A2.unwrap())*t + A1.unwrap())*t*cordic::exp(-x*x);
    return (Num::ONE+sign*y)/Into::<Num>::into(2);
}