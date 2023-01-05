fn main() {
    let (width, height, depth) : (i32, i32, i32) = (4, 7, 10);
    let area = area_of(width, height);
    println!("Area is {}", area);
    println!("=========================");
    let volume = volume(width, height, depth);
    println!("Volume is {}", volume);
}
fn volume(x: i32, y: i32, z: i32) -> i32 {
    x * y * z
}
fn area_of(x: i32, y: i32) -> i32{
    return x * y;
}