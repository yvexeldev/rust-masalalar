pub fn if1( son: i32) {
  let mut result = son;
  if son > 0 {
      result += 1;
  }
  println!("Son -> {} || Result -> {}", son, result);
}

pub fn if2( son: i32) {
  let mut result = son;
  if son > 0 {
      result += 1;
  }else if son < 0 {
      result -= 2;
  }
  println!("Son -> {} || Result -> {}", son, result);
}


pub fn if4(son1: i32, son2: i32, son3: i32) {
  let count: i32 = (son1 > 0) as i32 + (son2 > 0) as i32 + (son3 > 0) as i32;
  println!("Given Numbers -> {}, {}, {} || Result -> {}", son1, son2, son3, count)
}

pub fn if5(son1: i32, son2: i32, son3: i32) {
  let countp: i32 = (son1 > 0) as i32 + (son2 > 0) as i32 + (son3 > 0) as i32;
  let countm: i32 = (son1 < 0) as i32 + (son2 < 0) as i32 + (son3 < 0) as i32;
  
  println!(
      "Given Numbers -> {}, {}, {} || Result -> Musbat = {}, Manfiy = {}",
      son1, son2, son3, countp, countm
  );
}

pub fn if6(s1: i32, s2: i32) {
  println!("Given Numbers -> Num1 =  {}, Num2 = {} || Result -> {}",s1, s2,  if s1 > s2 { s1 } else { s2 })
}

pub fn if7(s1: i32, s2: i32) {
  println!("Given Numbers -> Num1 =  {}, Num2 = {} || Result -> {}",s1, s2,  if s1 > s2 { "1" } else { "2" })
}

pub fn if8(s1: i32, s2: i32) {
  println!("Given Numbers -> Num1 =  {}, Num2 = {} || Result -> {}",s1, s2,  if s1 > s2 { "2" } else { "1" })
}

pub fn if10(mut a: i32, mut b: i32) {
  let given_a = a;
  let given_b = b;
  if a != b {
      a += b;
      b = a;
  } else {
      b = 0;
      a = 0;
  }
  println!("Given numbers -> A = {}, B = {} || Result -> A = {}, B = {}",given_a, given_b, a, b)
}

pub fn if11(mut a: i32, mut b: i32) {
  if a != b {
      a = if a > b {a} else {b};
      b = a;
  } else {
      a = 0;
      b = a;
  } 
  println!("A -> {} || B -> {}", a, b)
}

pub fn if12( num1: i32,  num2: i32,  num3: i32){
  let mut smallest: i32 = num1;
  if smallest > num2 {
    smallest = num2;
  }
  if smallest > num3 {
    smallest = num3
  }
  println!("Given Number -> Num1 = {}, Num2 = {}, Num3 = {} || Smallest -> {}", num1, num2, num3, smallest)
}

pub fn if13( num1: i32,  num2: i32,  num3: i32){
  let middle: i32;
  if num1 > num2 && num2 > num3 || num3 > num2 && num2 > num1{
    middle = num2;
  }else if num2 > num1 && num1 > num3 || num3 > num1 && num1 > num2 {
    middle = num1;
  }else {
    middle = num3;
  }
  println!("Given Number -> Num1 = {}, Num2 = {}, Num3 = {} || Middle num -> {}", num1, num2, num3, middle);
}