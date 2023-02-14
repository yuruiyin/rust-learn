
pub fn test_struct() {
    println!("test struct");
    struct User {
        name: String,
        age: u8,
        school: String,
    }

    let user1 = User {
        name: String::from("yuruiyin"),
        age : 18,
        school: String::from("Tsinghua")
    };

    println!("name: {}, age: {}, school: {}", user1.name, user1.age, user1.school);

    let user2 = User {
        age: 20,
        ..user1
    };

    // 由于上面讲user1里头的字段赋值给了user2，里头的字段包括name和school，这个赋值相当于move操作，
    // 即将name和school对象的所有权交给了user2，这样user1就不再拥有这两个字段的所有权，因此不能再使用了，否则会报错
    println!("name: {}, age: {}, school: {}", user2.name, user2.age, user2.school);
}
