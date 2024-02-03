-------------------------------------------------------------------
                           Documentation
-------------------------------------------------------------------
# Gate

Vector to scalar regression primitive `uint(*)[] -> uint(*)`, with optimization based on probalistic mutation defined from feeback signal.

Parameters are optimized with mutagen (oriented mutation probability) to increase or decrease the parameter in function of mutagen sign.
Each time a mutation is done a anti mutation signal shall be propagated to other gates with dumping factor at each gate transfert.

## Feedback signal and mutation

Gates network shall be able back propagate a feedback `signal: float32`.

Each gate have a specific algorithm which define the 'feedback signal dispatch weight'

### Parameter based mutation (Solution A)

During propagation feedback signal, foreach parameter we compute a random value, if value is greater than mutation metric of parameter then parameter is modified, and some effect are applied on local network:
- a mutation damping signal will be co-propagated with the feedback signal propagation along this parameter, so the mutation damping signal shall be applied on mutation metric of parameter before comparaison with random value.
- this parameter is also flagged for self damping and forward mutation damping for the next feed forward propagation.

When a parameter mutate, it shall reduce the probability to mutate on short-term. It can be done by applying a damping factor on the probabilty to mutate.

When a parameter mutate, it's also a mutation of the local network around the parameter, so it shall reduce the probability to mutate of on short-term. It can be done by propagating a forward and backward mutation damping signal, the damping effect shall be reduced at each traversed link.

__Note__: parameter mutation can be seen like a distortion/burst in learned statistics of local network around it.

### Gate based mutation (Solution B)

TODO 


-------------------------------------------------------------------
## Interpolation Gate

### Parameters

Weights `w: uint8[n]`: weights used to interpolates inputs
Weights mutagens `w_mut: float32[n]`: mutation probability of `w`

### Inference

```
f(x) = sum(x[i]*w[i], i=0..n) , f: uint(*)[] -> uint16
```

### Backward propagation

TODO

-------------------------------------------------------------------
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

Feedback signal dispatch weight (DW):
```
b_dw = 1
w_dw[i] = signal * w[i] / sum(w[k], k=0..n)
```

Signal dispatch weight (DW):

Mutation statistics update:
```
b_mut += b_dw * signal
w_mut[i] += w_dw[i] * signal
```

-------------------------------------------------------------------
# Layer

Matrix to vector regression primitive `uint(*)[][] -> uint(*)[]`, with optimization based on probalistic mutation defined from feeback signal.

## Interpolation Layer



## Logic Layer

