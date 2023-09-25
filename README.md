# Rust-fb-yt

## Explain this code

这段代码使用了Firebase和HashMap库，并定义了两个结构体User和Response。然后通过一系列的异步函数来实现对Firebase数据库的操作。

### 代码的主要逻辑如下

1. 创建一个User结构体实例，并填充其字段。
2. 创建一个Firebase实例，指定数据库的URL。
3. 调用set_user函数将User数据存储到Firebase数据库，并获取返回的Response。
4. 调用get_user函数通过Response中的name字段获取特定用户的数据。
5. 打印获取到的用户数据。
6. 调用get_users函数获取所有用户的数据，并打印出来。
7. 更新用户数据的email字段。
8. 调用update_user函数将更新后的用户数据存储到Firebase数据库，并获取返回的更新后的User数据。
9. 打印更新后的用户数据。
10. 调用delete_user函数删除特定用户。
11. 打印"User deleted"。

代码中的每个异步函数都是对Firebase数据库的不同操作，包括设置用户数据、获取用户数据、更新用户数据和删除用户数据。这些函数都通过Firebase实例来访问数据库，并使用不同的方法来执行相应的操作。

在主函数中，首先创建一个User实例，然后创建一个Firebase实例，接着调用set_user函数将User数据存储到Firebase数据库，并获取返回的Response。然后通过response.name调用get_user函数获取特定用户的数据，并打印出来。接着调用get_users函数获取所有用户的数据，并打印出来。然后更新用户数据的email字段，并调用update_user函数将更新后的用户数据存储到Firebase数据库，并获取返回的更新后的User数据。最后调用delete_user函数删除特定用户，并打印"User deleted"。

这段代码实现了对Firebase数据库的基本操作，包括设置、获取、更新和删除用户数据。

## 问题解决

### 问题： firebase Permission denied

```shell
string s: "{\n  \"error\" : \"Permission denied\"\n}\n"
```

### 解决

1. Open firebase, select database on the left hand side.
2. Now on the right hand side, select [Realtime database] from the dropdown and change the rules to:

```shell
{
  "rules": {
    ".read": true,
    ".write": true
  }
}
```

### 更多参考

- <https://console.firebase.google.com/u/0/project/fb-rust-c9a1a/database/fb-rust-c9a1a-default-rtdb/rules?hl=zh-cn>
- <https://github.com/firebase/quickstart-js/issues/239>

## 运行

```shell
rust-fb-yt on  master [?] is 📦 0.1.0 via 🦀 1.72.1 via 🅒 base took 8.9s 
➜ cargo run
   Compiling rust-fb-yt v0.1.0 (/Users/qiaopengjun/Code/rust/rust-fb-yt)
    Finished dev [unoptimized + debuginfo] target(s) in 0.44s
     Running `target/debug/rust-fb-yt`
get_user, user: User { name: "John", age: 20, email: "john@example.com" }
get_users, users: {"-NfBH8aLVGDbM_5fY6AP": User { name: "John", age: 20, email: "john@example.com" }, "-NfBGcQG7XUKRddKyk-j": User { name: "John", age: 20, email: "johnupdate@example.com" }}
update_user, updated_user: User { name: "John", age: 20, email: "johnupdate@example.com" }
User deleted
