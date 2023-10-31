<a name="dpHAb"></a>
# 编程作业
<a name="MFtMI"></a>
## 实现过程

1. 给任务控制块添加两个字段 `start_time:usize`和 `syscall_times:[u8:MAX_SYSCALL_NUM]`来分别记录任务的启动时间和系统调用的记录表。（这里我给tcb实现了`Default`trait 来简化tcb初始化过程）
2. 在任务管理器中控制`start_time`的初始化（当`task_status`为 `Running`时进行初始化）。
3. 在系统调用时记录调用次数。
4. 通过当前的时间减去任务最开始运行的时间来获得任务运行时间的信息。
<a name="kWjig"></a>
# 简答作业	

1. 正确进入 U 态后，程序的特征还应有：使用 S 态特权指令，访问 S 态寄存器后会报错。 请同学们可以自行测试这些内容 (运行 [Rust 三个 bad 测例 (ch2b_bad_*.rs)](https://github.com/LearningOS/rCore-Tutorial-Test-2023A/tree/main/src/bin) ， 注意在编译时至少需要指定 `LOG=ERROR` 才能观察到内核的报错信息) ， 描述程序出错行为，同时注意注明你使用的 sbi 及其版本。

RustSBI version: 0.3.0-alpha.2
<a name="f2vvK"></a>
#### 错误信息
```bash
[kernel] PageFault in application, bad addr = 0x0, bad instruction = 0x80400414, kernel killed it.
[kernel] IllegalInstruction in application, kernel killed it.
[kernel] IllegalInstruction in application, kernel killed it.
```

2. 深入理解 [trap.S](https://github.com/LearningOS/rCore-Tutorial-Code-2023A/blob/ch3/os/src/trap/trap.S) 中两个函数 `__alltraps` 和 `__restore` 的作用，并回答如下问题:
   1. L40：刚进入 __restore 时，a0 代表了什么值。请指出 __restore 的两种使用情景。

`a0`保存了调用`__restore`时的第一个参数。 在这里代表了`TrapContext`上下文的地址。用于从特权态中恢复。

   2. L43-L48：这几行汇编代码特殊处理了哪些寄存器？这些寄存器的的值对于进入用户态有何意义？请分别解释。
```bash
ld t0, 32*8(sp)
ld t1, 33*8(sp)
ld t2, 2*8(sp)
csrw sstatus, t0
csrw sepc, t1
csrw sscratch, t2
```
特殊处理了`sstatus`, `sepc`, `sscratch`三个`csr`寄存器， 必须正确处理这些状态寄存器才能	完成特权级的切换与恢复。

      - `sstatus`标志了`CPU`所处的特权级
      -  `sepc`标志了`Trap`结束后的下一条指令地址
      - `sscratch`指向当前应用的内核栈栈顶

   1. L50-L56：为何跳过了 x2 和 x4？
```
ld x1, 1*8(sp)
ld x3, 3*8(sp)
.set n, 5
.rept 27
   LOAD_GP %n
   .set n, n+1
.endr
```
`x2`是栈顶指针， 此时它指向内核栈。 而用户栈指针已经被保存到`sscratch`。 `x4`是线程指针，一般不使用。

   2. L60：该指令之后，sp 和 sscratch 中的值分别有什么意义？

`csrrw sp, sscratch, sp`<br />该指令将`sp`和`sscratch`寄存器中的值进行交换，交换过后`sp`是用户栈指针， `sscratch`是内核栈指针。

   3. `__restore`：中发生状态切换在哪一条指令？为何该指令执行之后会进入用户态？

发生在`sret`指令，从中断处理程序返回到之前被中断的位置。

   4. L13：该指令之后，`sp` 和 `sscratch` 中的值分别有什么意义？	

`csrrw sp, sscratch, sp`<br />交换`sp`和`sscratch`寄存器中的值，使`sp`指向内核栈，`sscratch`指向用户栈。

   5. 从 U 态进入 S 态是哪一条指令发生的？

使用 `call` 指令调用陷阱处理程序 `trap_handler`, 调用后进入`stvec`寄存器的`Trap`地址开始执行。
