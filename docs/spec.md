-------------------------------------------------------------------
                           Documentation
-------------------------------------------------------------------
# Gate

Vector to scalar regression primitive `uint(*)[] -> uint(*)`, with optimization based on probalistic mutation defined from feeback signal.

Parameters are optimized with mutagen (oriented mutation probability) to increase or decrease the parameter in function of mutagen sign

## Interpolation Gate

### Parameters

Weights `w: uint8[n]`: weights used to interpolates inputs
Weights mutagens `w_mut: float32[n]`: mutation probability of `w`

### Inference

```
f(x) = sum(x[i]*w[i], i=0..n) , f: uint(*)[] -> uint16
```

### Backward propagation

Update mutation statistic with feedback signal `sig: float32`.

```
w_mut[i] += ...
```


## Logic Gate

### Parameters

Weights `w: uint8[n]`: weights used to interpolates inputs
Bias `b: uint32`: bias used to classify interpolation
Weights mutagens `w_mut: float32[n]`: mutation probability of `w`
Bias mutagens `b_mut: float32[n]`: mutation probability of `b`

### Inference

```
f(x) = sum(x[i]*w[i], i=0..n) > b , f: uint(*)[] -> uint1
```

### Backward propagation

m

-------------------------------------------------------------------
# Layer

Matrix to vector regression primitive `uint(*)[][] -> uint(*)[]`, with optimization based on probalistic mutation defined from feeback signal.

## Interpolation Layer



## Logic Layer

