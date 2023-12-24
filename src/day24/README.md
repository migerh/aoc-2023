The hail stones in our input lines are given as

p[i] and v[i] for 0 ≤ i < #lines

Then the stone we through is

p and v

For each hailstone there is one t[i] ≥ 0 so that

p + t[i] * v == p[i] + t[i] * v[i]

With some transformation we get

p - p[i] == t[i] * (v[i] - v)

⇒ p - p[i] and v - v[i] are parallel

That means

(p - p[i]) x (v - v[i]) == 0


Applying the cross product gives

```
  (p1-pi1)*(v2-vi2) - (p2-pi2)*(v1-vi1) == 0
  (p2-pi2)*(v0-vi0) - (p0-pi0)*(v2-vi2) == 0
  (p0-pi0)*(v1-vi1) - (p1-pi1)*(v0-vi0) == 0
```

```
   v                               v
  p1v2 - p1vi2 - pi1v2 + pi1vi2 - p2v1 + p2vi1 + pi2v1 - pi2vi1 == 0
  p2v0 - p2vi0 - pi2v0 + pi2vi0 - p0v2 + p0vi2 + pi0v2 - pi0vi2 == 0
  p0v1 - p0vi1 - pi0v1 + pi0vi1 - p1v0 + p1vi0 + pi1v0 - pi1vi0 == 0
```

Each equation has two summands that are not linear, e.g.:

```
  p1v2 - p2v1 - p1vi2 - pi1v2 + p2vi1 + pi2v1 + pi1vi2 - pi2vi1 == 0
  -----------
```

Using the first three hailstones p[i], v[i] with i ∈ { 0, 1, 2 } we can remove
these parts from our equations:

```
  - p1vA2 - pA1v2 + pA1vA2 + p2vA1 + pA2v1 - pA2vA1 == - p1vB2 - pB1v2 + pB1vB2 + p2vB1 + pB2v1 - pB2vB1
  - p2vA0 - pA2v0 + pA2vA0 + p0vA2 + pA0v2 - pA0vA2 == - p2vB0 - pB2v0 + pB2vB0 + p0vB2 + pB0v2 - pB0vB2
  - p0vA1 - pA0v1 + pA0vA1 + p1vA0 + pA1v0 - pA1vA0 == - p0vB1 - pB0v1 + pB0vB1 + p1vB0 + pB1v0 - pB1vB0

  - p1vA2 - pA1v2 + pA1vA2 + p2vA1 + pA2v1 - pA2vA1 == - p1vC2 - pC1v2 + pC1vC2 + p2vC1 + pC2v1 - pC2vC1
  - p2vA0 - pA2v0 + pA2vA0 + p0vA2 + pA0v2 - pA0vA2 == - p2vC0 - pC2v0 + pC2vC0 + p0vC2 + pC0v2 - pC0vC2
  - p0vA1 - pA0v1 + pA0vA1 + p1vA0 + pA1v0 - pA1vA0 == - p0vC1 - pC0v1 + pC0vC1 + p1vC0 + pC1v0 - pC1vC0
```

Bringing everything on the same side and grouping by unknowns:

```
         p0       p1       p2       v0       v1       v2 == RHS
  ---------------------------------------------------------------------------------------------
          0  vB2-vA2  vA1-vB1        0  pA2-pB2  pB1-pA1 == pB1vB2 - pB2vB1 + pA2vA1 - pA1vA2
    vA2-vB2        0  vB0-vA0  pB2-pA2        0  pA0-pB0 == pB2vB0 - pB0vB2 - pA2vA0 + pA0vA2
    vB1-vA1  vA0-vB0        0  pA1-pB1  pB0-pA0        0 == pB0vB1 - pB1vB0 - pA0vA1 + pA1vA0
          0  vC2-vA2  vA1-vC1        0  pA2-pC2  pC1-pA1 == pC1vC2 - pC2vC1 + pA2vA1 - pA1vA2
    vA2-vC2        0  vC0-vA0  pC2-pA2        0  pA0-pC0 == pC2vC0 - pC0vC2 - pA2vA0 + pA0vA2
    vC1-vA1  vA0-vC0        0  pA1-pC1  pC0-pA0        0 == pC0vC1 - pC1vC0 - pA0vA1 + pA1vA0
```
