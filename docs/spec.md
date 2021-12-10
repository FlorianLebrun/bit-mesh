-------------------------------------------------------------------
                           Documentation
-------------------------------------------------------------------
# Gate

Vector to scalar regression primitive `uint(*)[] -> uint(*)`, with optimization based on probalistic mutation defined from feeback signal.

## Interpolation Gate

### Inference

```
f(x) = sum(x[i]*w[i], i=0..n) , f: uint(*)[] -> uint16
```

### Backward propagation

Update mutation statistic with feedback signal


## Logic Gate

### Inference

```
f(x) = sum(x[i]*w[i], i=0..n) > bias , f: uint(*)[] -> uint1
```

### Backward propagation

m

-------------------------------------------------------------------
# Layer

Matrix to vector regression primitive `uint(*)[][] -> uint(*)[]`, with optimization based on probalistic mutation defined from feeback signal.

## Interpolation Layer



## Logic Layer

