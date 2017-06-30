mod cpp {
  #[link(name = "luajit-5.1")]
  extern "C" {
      pub fn tst() -> ::std::os::raw::c_int;
  }
}

pub fn tst() -> i32 {
  unsafe { cpp::tst() }
}
