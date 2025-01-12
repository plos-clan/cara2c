typedef signed char i8;
typedef short i16;
typedef int i32;
typedef long long i64;

typedef unsigned char u8;
typedef unsigned short u16;
typedef unsigned int u32;
typedef unsigned long long u64;

const i64 return_value = 0;
i8 test = 100;
void print(u64 num);
static i32 _4ece84f8c1a505905ed7_abcdefg_fn0_hijklmn() { 
return 0;
}
const i32 (*get_something) () = (const i32 (*) ())&_4ece84f8c1a505905ed7_abcdefg_fn0_hijklmn;
static i32 _608659ffe4fd8807d427_abcdefg_fn1_hijklmn() { 
i32 abcd = (((i32)return_value + (i32)test) + (get_something)());
((const void (*) (u64))&print)((u64)abcd);
return abcd;
}
const i32 (*main) () = (const i32 (*) ())&_608659ffe4fd8807d427_abcdefg_fn1_hijklmn;

