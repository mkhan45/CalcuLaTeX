# CalcuLaTeX

![Test Status](https://img.shields.io/github/workflow/status/mkhan45/CalcuLaTeX/Test?label=Tests)
![Crates.io](https://img.shields.io/crates/v/calculatex)

Try it out at <https://calcula.tech/app>

## Example

```
'''
Given a rock with mass $m_{rock}$ thrown by a force of $F_0$ acting over a duration $dt_{force}$,
calculate its airtime and the max height it achieves.
'''

''' Given: '''
g = 9.81 m/s^2
m_{rock} = 10 kg
F_0 = 1000 N
dt_{force} = 0.1 s


''' Solution: '''
y_{vel} = F_0 / m_{rock} * dt_{force} = ?

airtime = y_{vel} / g * 2 = ?
height_{max} = y_{vel} * airtime / 4 = ? cm
```

Outputs:

![example 1](images/ex1.png)

```
''' 
CalcuLaTeX:

Write an expression followed by = ? to evaluate it:
'''

50 g + 1 kg = ?

'''
You can suggest a unit if it outputs the wrong one. \\
Make sure it's correct otherwise the document won't \\
compile!
'''

5 m/s^2 * 3 kg = ?
5 m/s^2 * 3 kg = ? N

'''
Assign variables with =

They support some LaTeX formatting
'''
a_b = 5
c = 10 kg
a_b * c = ?
z = a_b kg + c = ?

'''
You can also add aliases for hard to type variables
'''
alias dx \frac{dx}{dt}
alias dt \Delta{t}

dx = 10 m/s
dt = 14 days
x = dx * dt = ?

'''
Set the output digits or use scientific notation \\
by using !digits \{n\} or !scientific 
'''

1 / 3 = ?

!digits 10
1 / 3 = ?

!digits 1
!scientific
1 kg = ? mg

'''
Basic mathematical functions are supported.

Additionally, builtin constants include $\pi$ and $e$.
'''

sin(\pi) = ?
cos(\pi) = ?
e = ?

'''
Escape raw latex with three single quotes.

There's still plenty of bugs, so feel free to \\
open an issue at \\
https://github.com/mkhan45/CalcuLaTeX
'''
```

Outputs:

![tutorial](images/tutorial.png)

___

# CLI Usage

Assuming the binary is called `calculatex`, running `calculatex [input] [output.pdf]` will watch the `input` file and output to `output.pdf` on change. This is meant for use with a PDF viewer which updates in realtime.

To run the CalcuLaTeX cli, you'll also need pandoc and a texlive installation.

___

# Compiling

To build CalcuLaTeX, you need a nightly Rust compiler and Cargo, preferably through rustup. After that, it's as simple as:

```
git clone git@github.com:mkhan45/CalcuLaTeX.git
cd CalcuLaTeX

# both of these output to target/release/
cargo build --release # builds the library only
cargo build --features build-binary --release # builds the CLI
```

___

# Contributing

If you'd like to contribute, feel free to open an issue or send me an email. The code base is very messy right now but it's small enough to be pretty understandable. Performance suggestions are welcome, but only if they don't introduce any extra complexity. I haven't benchmarked anything but LaTeX compilation is pretty much guaranteed to take an order of magnitude longer than the interpreter.

For guidelines, read [CONTRIBUTING.md](CONTRIBUTING.md)
