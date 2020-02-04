use time;

const e: f32 = 2.71828182845904523536028747135;

fn timestamp() -> f64 {
    let timespec = time::get_time();
    // 1459440009.113178
    let mills: f64 = timespec.sec as f64 + (timespec.nsec as f64 / 1000.0 / 1000.0 / 1000.0);
    mills
}
//fn:  x.powi(2) + 3.0*x.powi(2) -x.powi(4) - 0.5*x.powi(5) + 0.2*x.powi(6)
//pathological fn () ln(3 + e.powf(-20 + x) + e.powf(-20-x)).powi(2)
fn derivative(x: &f32) -> f32 {
   2.0*x + 3.0*2.0*x - 4.0*x.powi(3) - 0.5*5.0*x.powi(4) + 0.2*6.0*x.powi(5)
   //2.0*(e.powf(-20.0 + x) - e.powf(-20.0 - x))/(3.0 + e.powf(-20.0 + x) + e.powf(-20.0 - x))
}

fn derivative2(x: &f32) -> f32 {
    2.0 + 3.0*2.0*1.0 - 4.0*3.0*x.powi(2) - 0.5*5.0*4.0*x.powi(3) + 0.2*6.0*5.0*x.powi(4)
   // (2.0*(e.powf(-20.0 + x) + e.powf(-20.0 - x))/(3.0 + e.powf(-20.0 + x) + e.powf(-20.0 - x))) - (2.0*(e.powf(-20.0 + x) - e.powf(-20.0 - x)).powi(2)/((3.0 + e.powf(-20.0 + x) + e.powf(-20.0-x)).powi(2)))
}

fn main() {
    let x0 = 50.0;// We start the search at x=6
    let mut next_x: f32 = x0; 
    let step_size: f32 = 0.0001;
    let precision: f32 = 0.000001;
    let max_iterations: i32 = 10000;
    let mut out: f32 = x0;
    let mut time = timestamp();

    for i in 0..max_iterations {
        let current_x = next_x;
        next_x = current_x - step_size * derivative(&current_x);

        let step = next_x - current_x;
        println!("iteration: {}, current x: {}, step: {}", i, current_x, step);
        if step.abs() <= precision {
            out = current_x;
            break;
        }
    }
    println!("OUT:{}", out);
    println!("TIME  TAKEN:{}", (timestamp() - time));
    
    next_x = x0;
    time = timestamp();

    for i in 0..max_iterations {
        let current_x = next_x;
        next_x = current_x - (derivative(&current_x)/derivative2(&current_x));

        let step = next_x - current_x;
        println!("iteration: {}, current x: {}, step: {}", i, current_x, step);
        if step.abs() <= precision {
            out = current_x;
            break; 
        }
    }
    println!("OUT:{}", out);
    println!("TIME  TAKEN:{}", (timestamp() - time));
}