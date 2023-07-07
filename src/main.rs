mod functions;
use functions::{if1, if2, if4, if5, if6, if7, if8, if10, if11};

fn main() {
    println!("\n============ IF 1 ============");
    if1(1);
    if1(-12);
    if1(0);

    println!("\n============ IF 2 ============");
    if2(1);
    if2(-12);
    if2(0);

    println!("\n============ IF 4 ============");
    if4(1, 13, 15);
    if4(-12, 11, 18);
    if4(-56, -45, 34);
    if4(-17, -17, -22);

    println!("\n============ IF 5 ============");
    if5(1, 13, 15);
    if5(-12, 11, 18);
    if5(-56, -45, 34);
    if5(-17, -17, -22);

    println!("\n============ IF 6 ============");
    if6(1, 13);
    if6(-12, 11);
    if6(-56, -45);
    if6(-17, -12);

    println!("\n============ IF 7 ============");
    if7(17, 13);
    if7(-12, 11);
    if7(56, -45);
    if7(-17, -12);

    println!("\n============ IF 8 ============");
    if8(17, 13);
    if8(-12, 11);
    if8(56, -45);
    if8(-17, -12);

    println!("\n============ IF 10 ============");
    if10(17, 17);
    if10(17, 15);

    println!("\n============ IF 11 ============");
    if11(17, 17);
    if11(17, 15);


}
