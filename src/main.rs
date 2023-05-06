use std::collections::HashMap;
use std::fmt::Debug;

fn main() {
    let s2 = medium(vec![1,4,5,6,7,8,2,9,3,7,10,124,512]);
    println!("{:?}", s2);
    let s3 = mid(vec![1,4,5,6,7,8,2,9,3,7,10,435,436,23,56,876,45,87,90]);
    println!("{:?}", s3);
    let s1 = moda(vec![1,4,5,6,7,8,2,9,3,7,234,56,87,9,34]);
    println!("{:?}", s1);
}

fn medium(mut c: Vec<i32>) -> f32{
    let mut stac:i32 = 0;
    let mut count:i32 = 0;
    c.sort();
    for mut i in &c{
        count = count+ 1;
        stac = stac+i;
    };
    let med: f32 = stac as f32/ count as f32 ;
    med
}

fn mid(mut c: Vec<i32>) -> f32{
    c.sort();
    let count = c.len() as f32;
    if c.len()%2 == 0 {
        let mut firs_num:f32 = count/2.0;
        let second_num = &firs_num -1.0;
        let sum = (c[second_num as usize] as f32+c[firs_num as usize] as f32/2.0);
        sum
    }else {
        let middle = c[(count/2.0) as usize];
         middle as f32
    }

    }

fn moda(mut c: Vec<i32>)->i32{
    let mut map = HashMap::new();
    c.sort();
    for i in c{
        let count = map.entry(i).or_insert(0);
        *count+=1;

    }
    let mode = map.iter().max_by_key(|&(_, count)| count).map(|(&val, _)| val);
    mode.unwrap()
    }



