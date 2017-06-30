extern crate gcc;

fn main() {
  gcc::Config::new()
          .cpp(true)
          .pic(true)
          .flag("-std=c++11")
          .file("tst.cpp")
          .include("/usr/include/luajit-2.0")
          .compile("libtst.a");
}
