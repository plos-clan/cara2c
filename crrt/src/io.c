#include <stdint.h>
#include <stdarg.h>

/* 将整型转换成字符(integer to ascii) */
static void itoa(unsigned int value, char** buf_ptr_addr, unsigned char base) {
   unsigned int m = value % base;	    // 求模,最先掉下来的是最低位
   unsigned int i = value / base;	    // 取整
   if (i) {			    // 如果倍数不为0则递归调用。
      itoa(i, buf_ptr_addr, base);
   }
   if (m < 10) {      // 如果余数是0~9
      *((*buf_ptr_addr)++) = m + '0';	  // 将数字0~9转换为字符'0'~'9'
   } else {	      // 否则余数是A~F
      *((*buf_ptr_addr)++) = m - 10 + 'A'; // 将数字A~F转换为字符'A'~'F'
   }
}

/* 将参数ap按照格式format输出到字符串str,并返回替换后str长度 */
unsigned int vsprintf(int *back,char* str, const char* format, va_list ap) {
    unsigned int argc = 1;                   // 参数个数
    char* buf_ptr = str;
    const char* index_ptr = format;
    char index_char = *index_ptr;
    signed int arg_int;
    char* arg_str;
    while(index_char) {
        if (index_char != '%') {
            *(buf_ptr++) = index_char;
            index_char = *(++index_ptr);
            continue;
        }
        argc++;                          //参数变量个数累加
        index_char = *(++index_ptr);	 // 得到%后面的字符
        switch(index_char) {
            case 'd':
                arg_int = va_arg(ap, int);
                /* 若是负数, 将其转为正数后,再正数前面输出个负号'-'. */
                if (arg_int < 0) {
                    arg_int = 0 - arg_int;
                    *buf_ptr++ = '-';
                }
                itoa(arg_int, &buf_ptr, 10);
                index_char = *(++index_ptr);
                break;
            
            case 'l':
                arg_int = va_arg(ap, uint64_t);
                /* 若是负数, 将其转为正数后,再正数前面输出个负号'-'. */
                if (arg_int < 0) {
                    arg_int = 0 - arg_int;
                    *buf_ptr++ = '-';
                }
                itoa(arg_int, &buf_ptr, 10);
                index_char = *(++index_ptr);
                break;

            case 'x':
                arg_int = va_arg(ap, int);
                itoa(arg_int, &buf_ptr, 16);
                index_char = *(++index_ptr); // 跳过格式字符并更新index_char
                break;
            
            case 's':
                arg_str = va_arg(ap, char*);
                while(*arg_str) {
                    *(buf_ptr++) = *(arg_str++);
                }
                index_char = *(++index_ptr);
                break;
        }
    }

    *buf_ptr='\0';          //最后要打上目标字符串的结束标记。
    back[0]=argc;           //返回my_vsprintf的实际参数个数
    back[1]=buf_ptr-str;    //返回要打印的字符串长度
    return 0;
}

void write(long long fd,char *string, long long len)
{
    __asm__(
    "syscall"::"a"(1),"D"(fd),"S"(string),"d"(len)
    );
}

unsigned int printf(const char* format, ...)
{  int argc = 1;
   int len=0;
   int vsprintf_back[2]={0};
   va_list args;
   va_start(args, format);
   char buf[1024] = {0};
   vsprintf(vsprintf_back,buf, format, args);
   argc=vsprintf_back[0];
   len=vsprintf_back[1];

   write(1,buf, len);

   va_end(args);
   return argc;
}

void print(unsigned long long a) {
    printf("%l", a);
}

void putstr(char * str) {
    printf("%s", str);
}


