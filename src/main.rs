fn main() {
    let size_of_vec = 1000 ;
    let mut v  = Vec::<usize>::with_capacity(size_of_vec) ; // Конструирует новый пустой Vec<T> с как минимум указанной вместимостью.

    for i in 0..size_of_vec / 2{
        v.push(i);
    }

    println!("cap of v: {}, len of v: {}", v.capacity(), v.len()) ;

    v.shrink_to(600);  // Сокращает вместимость вектора с более низкой грницей.

    println!("cap of v: {}, len of v: {}", v.capacity(), v.len()) ;
} /* Out:
cap of v: 1000, len of v: 500
cap of v: 600, len of v: 500
*/


