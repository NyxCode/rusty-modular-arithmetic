# rusty-modular-arithmetic
###### Beautiful and animated multiplication tables on a circle
`n` (*adjustable with `ctrl + mousewheel`*) equally spaced points are put on the circumference of a circle. 
Now, a line is drawn from every point `p` to `(x * p) modulo n` (*adjust `x` with `mousewheel`*). 
Crazy, mind-blowing and gorgeous shapes and patterns start forming once you start playing around with `x`.

[Compiled binaries for Linux and Windows are available here](https://mega.nz/#F!uDoEXa5J!vAanF0nDEoLfqYOKYpVXMA)

  
###### `n = 100`, `x = 2`
![image](https://i.imgur.com/LVJMZs4.png)  

![image](https://i.imgur.com/GRWBQLG.png)

![image](https://i.imgur.com/d39Vt5f.png)

![image](https://i.imgur.com/777Dfjj.png)

## Key bindings
| Key            | Function                |
|----------------|-------------------------|
| `Space` or `A` | Start an animation      |
| `Shift`        | Accelerate              |
| `Ctrl`         | Change the subdivisions |
| `Alt`          | Reverse the animation   |
| `Escape`       | Reset                   |

## Run
```
git clone https://github.com/NyxCode/rusty-modular-arithmetic
cd rusty-modular-arithmetic
cargo run
```
*note: On `Wayland`, run with `ALPHA_BLENDING=false cargo run`*
