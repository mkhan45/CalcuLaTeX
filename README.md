# math_lang
pretty printing calculator language

Kind of like smart LaTeX

needs a better name

## Example

```
mass = 10 kg

F_0 = 1000 N = ?
dt_{force} = 0.5 s

y_{vel} = F_0 / mass * dt_{force} = ?

g = 9.81 m/s^2
dt_{air} = 10 s

h = y_{vel} * dt_{air} - ((1/2) * g * dt_{air}^2) = ?
```

Outputs:

![example 1](images/ex1.png)

___

```
ln2 = 0.693

k = ln2 / (4.47 * 10^9 * 1 year) = ?

e = 2.71828

hl = 4.47 year * 10^9
t_{est} = 3.8 year * 10^9

ntn0 = 1 / (e^(k * t_{est})) = ?

mass = 1.515 g

x = (mass - (ntn0 * mass)) / ntn0 = ?

x * ((1 mol) / (238 g)) * ((206 g) / (1 mol)) = ?
```

Outputs:


## Compiling:

Requires GMP, MPFR, and MPC development libraries: <https://docs.rs/gmp-mpfr-sys/1.4.3/gmp_mpfr_sys/index.html>
