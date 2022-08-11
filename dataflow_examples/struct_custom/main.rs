use ::libc;
extern "C" {
    fn pthread_mutex_lock(__mutex: *mut pthread_mutex_t) -> libc::c_int;
    fn pthread_mutex_unlock(__mutex: *mut pthread_mutex_t) -> libc::c_int;
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __pthread_internal_list {
    pub __prev: *mut __pthread_internal_list,
    pub __next: *mut __pthread_internal_list,
}
pub type __pthread_list_t = __pthread_internal_list;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __pthread_mutex_s {
    pub __lock: libc::c_int,
    pub __count: libc::c_uint,
    pub __owner: libc::c_int,
    pub __nusers: libc::c_uint,
    pub __kind: libc::c_int,
    pub __spins: libc::c_short,
    pub __elision: libc::c_short,
    pub __list: __pthread_list_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union __anonunion_pthread_mutex_t_335460617 {
    pub __data: __pthread_mutex_s,
    pub __size: [libc::c_char; 40],
    pub __align: libc::c_long,
}
pub type pthread_mutex_t = __anonunion_pthread_mutex_t_335460617;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __anonstruct_ss_672045599 {
    pub n: libc::c_int,
    pub m: pthread_mutex_t,
}
pub type ss = __anonstruct_ss_672045599;
pub unsafe extern "C" fn lock(mut s: *mut ss) {
    pthread_mutex_lock(&mut (*s).m);
}
pub unsafe extern "C" fn unlock(mut s: *mut ss) {
    pthread_mutex_unlock(&mut (*s).m);
}
pub unsafe extern "C" fn lock1(mut t: *mut ss) {
    lock(t);
}
pub unsafe extern "C" fn unlock1(mut t: *mut ss) {
    unlock(t);
}
pub unsafe extern "C" fn lock2(mut r: *mut ss) {
    lock1(r);
}
pub unsafe extern "C" fn unlock2(mut r: *mut ss) {
    unlock1(r);
}
pub unsafe extern "C" fn f1(mut u: *mut ss) {
    lock(u);
    (*u).n += 1;
    unlock(u);
    lock1(u);
    (*u).n += 1;
    unlock1(u);
    lock2(u);
    (*u).n += 1;
    unlock2(u);
}
pub unsafe extern "C" fn f2(mut s: *mut ss) {
    let mut u: ss = ss {
        n: 0,
        m: __anonunion_pthread_mutex_t_335460617 {
            __data: __pthread_mutex_s {
                __lock: 0,
                __count: 0,
                __owner: 0,
                __nusers: 0,
                __kind: 0,
                __spins: 0,
                __elision: 0,
                __list: __pthread_list_t {
                    __prev: 0 as *mut __pthread_internal_list,
                    __next: 0 as *mut __pthread_internal_list,
                },
            },
        },
    };
    u = *s;
    lock(&mut u);
    u.n += 1;
    unlock(&mut u);
    lock1(&mut u);
    u.n += 1;
    unlock1(&mut u);
    lock2(&mut u);
    u.n += 1;
    unlock2(&mut u);
}
unsafe fn main_0() -> libc::c_int {
    return 0 as libc::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}
