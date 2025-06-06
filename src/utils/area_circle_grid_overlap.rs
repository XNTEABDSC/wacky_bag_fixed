
use std::collections::HashMap;

use fixed::traits::Fixed;

use crate::utils::area_circle_polygon_overlap::area_circle_rectangle;
use crate::num::*;
use crate::vec2_fix::*;

pub fn area_circle_grid_overlap<TFn>(pos:Vec2Fix,radius:Num,mut on_result:TFn)
    where TFn:FnMut((i32,i32),Num)
{
    let (x,y)=(pos[0],pos[1]);

    let bottom=y-radius;
    let bottomfi=Num::floor(bottom);
    let radius_sq=radius*radius;

    struct LineData{
        y_f:Num,
        y_int:i32,
        y_to_mid:Num,
        x_to_mid:Num,
    }

    let get_line_data=|y_f:Num,y_int:i32|{
        let y_to_mid=y-y_f;
        
        let x_to_mid=if y_to_mid.abs()<radius { Num::sqrt(radius_sq-y_to_mid*y_to_mid) } else{ Num::ZERO };
        return LineData{
            y_f,y_int,y_to_mid,x_to_mid
        };
    };

    let mut cur_i_data=get_line_data(bottomfi,bottomfi.to_num());
    let top=y+radius;
    let top_ci=Num::ceil(top);
    while cur_i_data.y_int<top_ci {
        let iy=cur_i_data.y_int;
        let iyf=cur_i_data.y_f;
        let iyf_y=iyf-y;

        let cur_i2=cur_i_data.y_int+1;
        let cur_i2_data=get_line_data(Num::from_num(cur_i2),cur_i2);
        let iyf_2_y=cur_i2_data.y_f-y;

        let at_mid=cur_i_data.y_int<y && y<cur_i2_data.y_int;
        let (x_edge_min_to_mid,x_edge_max_to_mid)=if cur_i_data.x_to_mid< cur_i2_data.x_to_mid{
            (cur_i_data.x_to_mid,  if at_mid {radius} else {cur_i2_data.x_to_mid})
        }else{
            (cur_i2_data.x_to_mid, if at_mid {radius} else {cur_i_data.x_to_mid})
        };
        let x_edge_1=x-x_edge_max_to_mid;
        let x_edge_2=x-x_edge_min_to_mid;
        let x_edge_3=x+x_edge_min_to_mid;
        let x_edge_4=x+x_edge_max_to_mid;
        let x_edge_1_int:i32=x_edge_1.floor().to_num();
        let x_edge_2_int:i32=x_edge_2.ceil().to_num();
        let x_edge_3_int:i32=x_edge_3.floor().to_num();
        let x_edge_4_int:i32=x_edge_4.ceil().to_num();

        for ix in x_edge_1_int..x_edge_2_int {
            let ixf=Num::from_num(ix)-x;
            on_result((ix,iy),area_circle_rectangle(radius, radius_sq, ixf, iyf_y, ixf+Num::ONE, iyf_2_y));
        }
        for ix in x_edge_2_int..x_edge_3_int {
            on_result((ix,iy),Num::ONE);
        }
        for ix in x_edge_3_int..x_edge_4_int {
            let ixf=Num::from_num(ix)-x;
            on_result((ix,iy),area_circle_rectangle(radius, radius_sq, ixf, iyf_y, ixf+Num::ONE, iyf_2_y));
        }
        
        cur_i_data=cur_i2_data;
        //let x_dist_sq=
    }
}

fn area_circle_grid_overlap_test<TFn>(pos:Vec2Fix,radius:Num,mut on_result:TFn)
    where TFn:FnMut((i32,i32),Num)
{
    let r = radius;
    let r_sq = r*r;

    // Helper function to compute floor of a fixed-point number
    fn floor(n:Num)->i32{Num::floor(n).to_num()}

    // Calculate the bounding box of the circle in world coordinates
    let c_min_x = pos[0] - r;
    let c_max_x = pos[0] + r;
    let c_min_y = pos[1] - r;
    let c_max_y = pos[1] + r;

    // Compute the range of cell indices in x and y directions
    let i_min = floor(c_min_x);
    let i_max = floor(c_max_x);
    let j_min = floor(c_min_y);
    let j_max = floor(c_max_y);

    // Iterate over each cell in the computed ranges
    for i in i_min..=i_max {
        for j in j_min..=j_max {
            // Translate cell coordinates to the circle's local system
            let x1 = Num::from(i) - pos[0];
            let y1 = Num::from(j) - pos[1];
            let x2 = Num::from(i + 1) - pos[0];
            let y2 = Num::from(j + 1) - pos[1];

            // Calculate the overlapping area
            let area = area_circle_rectangle(r, r_sq, x1, y1, x2, y2);

            if area > Num::from(0) {
                on_result((i, j), area);
            }
        }
    }
}
#[test]
fn test(){

    let test_cases = [
            // Circle centered at (0.0, 0.0) with radius 1.0
            (Vec2Fix::new(Num::from_num(0), Num::from_num(0)), Num::from_num(1.0)),
            // Circle at (0.5, 0.5) with small radius
            (Vec2Fix::new(Num::from_num(0.5), Num::from_num(0.5)), Num::from_num(0.4)),
            // Circle in negative coordinates
            (Vec2Fix::new(Num::from_num(-1.2), Num::from_num(0.8)), Num::from_num(0.6)),
            // Circle in negative coordinates
            (Vec2Fix::new(Num::from_num(-10), Num::from_num(10)), Num::from_num(5)),
            // Circle in negative coordinates
            (Vec2Fix::new(Num::from_num(-10), Num::from_num(10)), Num::from_num(20)),
        ];

    fn test_inner(pos:Vec2Fix,radius:Num){
        
        let mut hma=HashMap::<(i32,i32),Num>::new();
        area_circle_grid_overlap(pos,radius,|p,a|{
            hma.insert(p, a);
        });
        area_circle_grid_overlap_test(pos,radius,|p,a|{
            let hmares=hma.get(&p);
            if let Some(ahma) = hmares{
                if (a-*ahma).abs()>Num::DELTA*16 {
                    panic!("testing pos: {pos:?}, rad:{radius}, at pos:{p:?}, result: {ahma}, {a}");
                }else {
                    hma.remove(&p);
                }
            }else{
                if a>Num::DELTA*16{
                    panic!("testing pos: {pos:?}, rad:{radius}, at pos:{p:?}, result: 0, {a}");
                }
            }
            //hma.insert(p, a);
        });
        for (p,a) in hma.iter() {
            if *a>Num::DELTA*16 {
                panic!("testing pos: {pos:?}, rad:{radius}, at pos:{p:?}, result: {a}, 0");
            }
        }

    }
    for (pos, radius) in test_cases {
        test_inner(pos,radius)
    }
}