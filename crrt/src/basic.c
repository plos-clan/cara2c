void exit(int code){
    __asm__(
    "syscall"::"D"(code),"a"(60)
    );
}

